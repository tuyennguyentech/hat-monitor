use std::time::Duration;

use rumqttc::v5::{AsyncClient, MqttOptions};
use tracing::debug;

pub(crate) async fn run(topic: &str) {
  let mut mqtt_options = MqttOptions::new("hat-monitor", "localhost", 1883);
  mqtt_options.set_keep_alive(Duration::from_secs(5));

  let (client, mut event_loop) = AsyncClient::new(mqtt_options, 1000);
  client
    .subscribe(topic, rumqttc::v5::mqttbytes::QoS::AtLeastOnce)
    .await.unwrap();

  loop {
    match event_loop.poll().await {
      Ok(event) => {
        debug!(targer = "event_loop", case = "ok", "{:?}", event);
      }
      Err(error) => {
        debug!(targer = "event_loop", case = "err", "{:?}", error);
      }
    }
  }
}
