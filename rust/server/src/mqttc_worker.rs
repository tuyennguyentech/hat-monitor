use std::time::Duration;

use chrono::Utc;
use rand::{Rng, SeedableRng};
use rumqttc::v5::{mqttbytes::QoS, AsyncClient, Event, Incoming, MqttOptions};
use tokio::{
  sync::watch::{self},
  task, time,
};
use tracing::{debug, warn};
use types::HatSample;

pub(crate) async fn run(topic: &str, tx: watch::Sender<HatSample>) {
  let mut mqtt_options = MqttOptions::new("hat-monitor", "localhost", 1883);
  mqtt_options.set_keep_alive(Duration::from_secs(5));
  let (client, mut event_loop) = AsyncClient::new(mqtt_options, 1000);
  client.subscribe(topic, QoS::AtLeastOnce).await.unwrap();
  let publish_client = client.clone();
  let publish_topic = topic.to_string();
  task::spawn(task::coop::cooperative(async move {
    let mut rng = rand::rngs::StdRng::from_rng(&mut rand::rng());
    let mut interval = time::interval(Duration::from_secs(5));
    loop {
      interval.tick().await;
      let sample = HatSample {
        timestamp: Utc::now().timestamp() as u64,
        temperature: rng.random_range(27.0..=33.0),
        humidity: rng.random_range(50.0..=55.5),
        r_zero: rng.random_range(0.0..=50.0),
        corrected_r_zero: rng.random_range(0.0..=50.0),
        resistance: rng.random_range(0.0..=100.0),
        ppm: rng.random_range(200.0..=300.0),
        corrected_ppm: rng.random_range(200.0..=300.0),
      };
      let payload = serde_json::to_string(&sample).expect("should be serialized");
      debug!(target = "publish", "{:#?}", payload);
      publish_client
        .publish(&publish_topic, QoS::AtLeastOnce, false, payload)
        .await
        .expect("should be published");
    }
  }));

  loop {
    match event_loop.poll().await {
      Ok(Event::Incoming(Incoming::Publish(publish))) => {
        let hat_sample = serde_json::from_slice::<HatSample>(&publish.payload)
          .inspect_err(|e| warn!(target = "event_loop", case = "publish", "{:?}", e))
          .unwrap_or_default();
        debug!(target = "event_loop", case = "publish", "{:#?}", hat_sample);
        let _ = tx
          .send(hat_sample)
          .inspect_err(|e| warn!(target = "event_loop", case = "publish", "{:?}", e));
      }
      Ok(event) => {
        debug!(targer = "event_loop", case = "ok", "{:?}", event);
      }
      Err(error) => {
        debug!(targer = "event_loop", case = "err", "{:?}", error);
      }
    }
  }
}
