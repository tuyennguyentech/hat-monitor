mod mqttc_worker;

use app::*;
use axum::Router;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use tokio::join;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

  let app = Router::new()
    .leptos_routes(&leptos_options, routes, {
      let leptos_options = leptos_options.clone();
      move || shell(leptos_options.clone())
    })
    .fallback(leptos_axum::file_and_error_handler(shell))
    .with_state(leptos_options)
    .layer(TraceLayer::new_for_http());

  // run our app with hyper
  // `axum::Server` is a re-export of `hyper::Server`
  log!("listening on http://{}", &addr);
  let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
  let ret = join!(
    mqttc_worker::run("iot/hat"),
    axum::serve(listener, app.into_make_service())
  );
  ret.1.unwrap();
}
