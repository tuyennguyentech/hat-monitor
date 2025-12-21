use chrono::{offset::LocalResult, FixedOffset, TimeZone};
use leptos::{prelude::*, server::codee::string::JsonSerdeCodec};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
  components::{Route, Router, Routes},
  StaticSegment,
};
use leptos_use::{core::ConnectionReadyState, use_websocket, UseWebSocketReturn};
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
  // Creates a reactive value to update the button
  // let count = RwSignal::new(0);
  // let on_click = move |_| *count.write() += 1;

  view! {
    <div class="flex flex-col items-center justify-center h-screen bg-base-100 gap-8">
      <h1 class="text-5xl leading-normal font-black bg-clip-text text-transparent bg-linear-to-r from-primary to-secondary">
        "Humidity, air quality and temperature monitor"
      </h1>

      // <div class="stats shadow-lg bg-base-200 border border-base-300">
      // <div class="stat place-items-center px-10 py-6">
      // <div class="stat-title text-lg uppercase tracking-widest">"Total Clicks"</div>

      // <div class="stat-value text-primary text-6xl my-2 font-mono">{count}</div>

      // <div class="stat-desc">"Click to increase value"</div>
      // </div>
      // </div>

      // // Nút bấm tách rời
      // <button on:click=on_click class="btn btn-outline btn-primary btn-wide gap-2">
      // <svg
      // xmlns="http://www.w3.org/2000/svg"
      // fill="none"
      // viewBox="0 0 24 24"
      // stroke-width="1.5"
      // stroke="currentColor"
      // class="w-6 h-6"
      // >
      // <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
      // </svg>
      // "Increase Counter"
      // </button>
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

  // --- LOGIC MÀU SẮC ---

  // 1. Logic màu Nhiệt độ
  let temp_class = move || {
    message
      .get()
      .map(|s| {
        if s.temperature > 30.0 {
          "text-error"
        }
        // > 30: Đỏ
        else if s.temperature < 20.0 {
          "text-info"
        }
        // < 20: Xanh dương
        else {
          "text-success"
        } // Bình thường: Xanh lá
      })
      .unwrap_or("text-base-content")
  };

  // 2. Logic màu Chất lượng không khí (PPM)
  // Giả định: < 1000 là Tốt, > 2000 là Kém
  let ppm_class = move || {
    message
      .get()
      .map(|s| {
        if s.corrected_ppm > 2000.0 {
          "text-error"
        }
        // Nguy hiểm
        else if s.corrected_ppm > 1000.0 {
          "text-warning"
        }
        // Cảnh báo
        else {
          "text-success"
        } // Tốt
      })
      .unwrap_or("text-base-content")
  };

  // --- HELPER VIEW ---
  let connection_badge = move || match ready_state.get() {
    ConnectionReadyState::Open => view! { <div class="badge badge-success gap-2">"Online"</div> },
    ConnectionReadyState::Closed => view! { <div class="badge badge-error gap-2">"Offline"</div> },
    _ => view! { <div class="badge badge-warning gap-2">"Connecting..."</div> },
  };

  view! {
        <div class="p-4 flex flex-col items-center gap-6 w-full">
            // Header trạng thái
            <div class="flex items-center gap-2">
                <h2 class="text-xl font-bold">"Giám sát Môi trường"</h2>
                {connection_badge}
            </div>

            // CONTAINER STATS CHÍNH
            // stats-vertical: Mặc định xếp dọc (cho mobile)
            // lg:stats-horizontal: Màn hình lớn sẽ xếp ngang
            <div class="stats stats-vertical lg:stats-horizontal shadow bg-base-100 w-full max-w-4xl border border-base-200">

                // --- Ô 1: NHIỆT ĐỘ ---
                <div class="stat">
                    <div class="stat-figure text-secondary">
                        // Icon Nhiệt kế
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
                           <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m9-.75a9 9 0 11-18 0 9 9 0 0118 0zm-9 3.75h.008v.008H12v-.008z" />
                        </svg>
                    </div>
                    <div class="stat-title">"Nhiệt độ"</div>
                    <div class={move || format!("stat-value {}", temp_class())}>
                        {move || match message.get() {
                            Some(s) => format!("{:.1}°C", s.temperature),
                            None => "--".to_string(),
                        }}
                    </div>
                    <div class="stat-desc">"Ngưỡng an toàn: 20-30°C"</div>
                </div>

                // --- Ô 2: ĐỘ ẨM ---
                <div class="stat">
                    <div class="stat-figure text-info">
                        // Icon Giọt nước
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 21a9.004 9.004 0 008.716-6.747M12 21a9.004 9.004 0 01-8.716-6.747M12 21c2.485 0 4.5-4.03 4.5-9S12 5.625 12 5.625 7.5 7.375 7.5 12S9.515 21 12 21z" />
                        </svg>
                    </div>
                    <div class="stat-title">"Độ ẩm"</div>
                    <div class="stat-value text-info">
                        {move || match message.get() {
                            Some(s) => format!("{:.1}%", s.humidity),
                            None => "--".to_string(),
                        }}
                    </div>
                    <div class="stat-desc">"Độ ẩm tương đối (RH)"</div>
                </div>

                // --- Ô 3: KHÔNG KHÍ (PPM) ---
                <div class="stat">
                    <div class="stat-figure text-accent">
                         // Icon Đám mây/Gas
                         <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-8 h-8">
                             <path stroke-linecap="round" stroke-linejoin="round" d="M12 16.5V9.75m0 0l3 3m-3-3l-3 3M6.75 19.5a4.5 4.5 0 01-1.41-8.775 5.25 5.25 0 0110.233-2.33 3 3 0 013.758 3.848A3.752 3.752 0 0118 19.5H6.75z" />
                         </svg>
                    </div>
                    <div class="stat-title">"Chất lượng khí (CO2)"</div>
                    <div class={move || format!("stat-value {}", ppm_class())}>
                        {move || match message.get() {
                            Some(s) => format!("{:.0} PPM", s.corrected_ppm),
                            None => "--".to_string(),
                        }}
                    </div>
                    // Hiển thị thêm thông số kỹ thuật (Resistance) ở phần mô tả
                    <div class="stat-desc text-xs">
                        {move || match message.get() {
                            Some(s) => format!("R: {:.0} Ω | R0: {:.0} Ω", s.resistance, s.corrected_r_zero),
                            None => "Đang hiệu chuẩn...".to_string(),
                        }}
                    </div>
                </div>

            </div>

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
                    let s = message.get().unwrap(); // Lấy dữ liệu ra
                    view! {
                        <span class="flex items-center gap-1">
                            // ... Icon ...
                            "Cập nhật lúc: "
                            <span class="font-mono font-bold">
                                {format_vn_timestamp(s.timestamp)}
                            </span>
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
