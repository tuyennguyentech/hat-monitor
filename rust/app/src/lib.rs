mod connection_badge;
mod humidity;
mod ppm;
mod temperature;
mod graph;

use chrono::{offset::LocalResult, FixedOffset, TimeZone};
use connection_badge::ConnectionBadge;
use leptos::{prelude::*, server::codee::string::JsonSerdeCodec};
use leptos_meta::{provide_meta_context, MetaTags, Script, Stylesheet, Title};
use leptos_router::{
  components::{Route, Router, Routes},
  StaticSegment,
};
use leptos_use::{
  use_websocket, UseWebSocketReturn,
};
use types::HatSample;

pub fn shell(options: LeptosOptions) -> impl IntoView {
  view! {
    <!DOCTYPE html>
    <html lang="en" data-theme="dracula">
      <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <AutoReload options=options.clone() />
        <HydrationScripts options />
        <MetaTags />
      </head>
      <body>
        <App />
      </body>
    </html>
  }
}

#[component]
pub fn App() -> impl IntoView {
  // Provides context that manages stylesheets, titles, meta tags, etc.
  provide_meta_context();

  view! {
    <Stylesheet id="hat-monitor" href="/pkg/hat-monitor.css" />

    // sets the document title
    <Title text="Welcome to humidity, air quality and temperature monitor" />

    <Script src="https://cdn.jsdelivr.net/npm/echarts@5.5.1/dist/echarts.min.js" />
    <Script src="https://cdn.jsdelivr.net/npm/echarts-gl@2.0.9/dist/echarts-gl.min.js" />

    // content for this welcome page
    <Router>
      <main>
        <Routes fallback=|| "Page not found.".into_view()>
          <Route path=StaticSegment("") view=HomePage />
        </Routes>
      </main>
    </Router>
  }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
  view! {
    <div class="flex flex-col items-center justify-start h-screen bg-base-100 gap-8">
      <h1 class="text-5xl leading-normal font-black bg-clip-text text-transparent bg-linear-to-r from-primary to-secondary text-center">
        "Humidity, air quality and temperature monitor"
      </h1>
      <Monitor />
    </div>
  }
}

#[component]
fn Monitor() -> impl IntoView {
  let UseWebSocketReturn {
    message,
    ready_state,
    ..
  } = use_websocket::<HatSample, HatSample, JsonSerdeCodec>("ws://localhost:3000/ws");

  view! {
    <div class="p-4 flex flex-col flex-wrap items-center gap-6 w-full">
      // Header trạng thái
      <div class="flex items-center gap-2">
        <ConnectionBadge ready=ready_state />
      </div>

      // CONTAINER STATS CHÍNH
      // stats-vertical: Mặc định xếp dọc (cho mobile)
      // lg:stats-horizontal: Màn hình lớn sẽ xếp ngang
      <div class="stats stats-vertical lg:stats-horizontal shadow bg-base-100 w-full max-w-4xl border border-base-200">

        <temperature::Temperature sample=message.clone() />

        <humidity::Humidity sample=message.clone() />

        <ppm::Ppm sample=message.clone() />

      </div>
      <temperature::Graph sample=message.clone() />
      <humidity::Graph sample=message.clone() />
      <ppm::Graph sample=message.clone() />
      // <Graph />

      // Footer hiển thị Timestamp cập nhật lần cuối
      <Show
        // Điều kiện: Chỉ hiển thị children khi có dữ liệu (Some)
        when=move || message.get().is_some()
        // Fallback: Hiển thị khi không có dữ liệu (None)
        fallback=|| view! { <span class="italic text-gray-500">"Đang chờ dữ liệu..."</span> }
      >
        // Phần hiển thị khi có dữ liệu
        // Vì đã check is_some() ở trên, ta có thể unwrap an toàn hoặc dùng with()
        {move || {
          let s = message.get().unwrap();
          // Lấy dữ liệu ra
          view! {
            <span class="flex items-center gap-1">
              // ... Icon ...
              "Cập nhật lúc: "
              <span class="font-mono font-bold">{format_vn_timestamp(s.timestamp)}</span>
            </span>
          }
        }}
      </Show>
    </div>
  }
}

fn format_vn_timestamp(ts: u64) -> String {
  // 1. Tạo múi giờ Việt Nam (UTC+7)
  // 7 giờ * 3600 giây/giờ = 25200 giây
  let vn_offset = FixedOffset::east_opt(7 * 3600).unwrap();

  // 2. Chuyển timestamp (giây) sang đối tượng DateTime
  // Lưu ý: Nếu timestamp của bạn là mili-giây, hãy dùng `ts as i64 / 1000`
  if let LocalResult::Single(dt) = vn_offset.timestamp_opt(ts as i64, 0) {
    // 3. Format theo kiểu Việt Nam: Ngày/Tháng/Năm Giờ:Phút:Giây
    dt.format("%d/%m/%Y %H:%M:%S").to_string()
  } else {
    "Invalid Time".to_string()
  }
}
