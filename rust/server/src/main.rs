mod mqttc_worker;

use app::*;
use axum::{
  extract::{
    ws::{Message, WebSocket},
    State, WebSocketUpgrade,
  },
  response::IntoResponse,
  routing::any,
  Router,
};
use leptos::logging::log;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use tokio::{join, select, sync::watch};
use tower_http::trace::TraceLayer;
use tracing::{debug, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use types::HatSample;

const MQTT_TOPIC: &'static str = "iot/hat";

#[tokio::main]
async fn main() {
  tracing_subscriber::registry()
    .with(
      tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        // axum logs rejections from built-in extractors with the `axum::rejection`
        // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
        format!(
          "{}=debug,tower_http=debug,axum::rejection=trace",
          env!("CARGO_CRATE_NAME")
        )
        .into()
      }),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

  let conf = get_configuration(None).unwrap();
  let addr = conf.leptos_options.site_addr;
  let leptos_options = conf.leptos_options;
  // Generate the list of routes in your Leptos App
  let routes = generate_route_list(App);

  let (tx, rx) = watch::channel(HatSample::default());
  for i in 0..2 {
    let mut rx = rx.clone();
    tokio::spawn(async move {
      loop {
        let hat_sample = rx.borrow_and_update().clone();
        debug!("receiver {}: {:?}", i, hat_sample);
        if rx.changed().await.is_err() {
          debug!("stop {}", i);
          break;
        }
      }
    });
  }

  let app = Router::new()
    .leptos_routes(&leptos_options, routes, {
      let leptos_options = leptos_options.clone();
      move || shell(leptos_options.clone())
    })
    .fallback(leptos_axum::file_and_error_handler(shell))
    .with_state(leptos_options)
    .route("/ws", any(ws_handler))
    .with_state(rx)
    // .route(path, method_router)
    .layer(TraceLayer::new_for_http());

  // run our app with hyper
  // `axum::Server` is a re-export of `hyper::Server`
  log!("listening on http://{}", &addr);
  let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
  let ret = join!(
    mqttc_worker::run(MQTT_TOPIC, tx),
    axum::serve(listener, app.into_make_service())
  );
  ret.1.unwrap();
}

async fn ws_handler(
  ws: WebSocketUpgrade,
  State(rx): State<watch::Receiver<HatSample>>,
) -> impl IntoResponse {
  ws.on_upgrade(|socket| handle_ws(socket, rx))
}

async fn handle_ws(mut socket: WebSocket, mut rx: watch::Receiver<HatSample>) {
  // serde_json::to_string(value)
  loop {
    select! {
      Some(msg) = socket.recv() => {
        let Ok(_) = msg else {
          break;
        };
      },
      changed = rx.changed() => {
        if changed.is_err() {
          break;
        }
        let sample = rx.borrow().clone();
        let json = match serde_json::to_string(&sample) {
          Ok(json) => json,
          Err(e) => {
            warn!(target = "handle_ws", case = "serde json err", "{:?}", e);
            continue;
          }
        };
        if socket.send(Message::Text(json.into())).await.is_err() {
          break;
        }
      }
      else => break,
    }
  }
}
