use std::time::Duration;

use rumqttc::v5::{AsyncClient, Event, Incoming, MqttOptions};
use tokio::sync::watch::{self};
use tracing::{debug, warn};
use types::HatSample;

pub(crate) async fn run(topic: &str, tx: watch::Sender<HatSample>) {
  let mut mqtt_options = MqttOptions::new("hat-monitor", "localhost", 1883);
  mqtt_options.set_keep_alive(Duration::from_secs(5));
  let (client, mut event_loop) = AsyncClient::new(mqtt_options, 1000);
  client
    .subscribe(topic, rumqttc::v5::mqttbytes::QoS::AtLeastOnce)
    .await
    .unwrap();

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
