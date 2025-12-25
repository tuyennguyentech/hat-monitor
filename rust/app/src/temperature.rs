use std::collections::VecDeque;

use charming::{
  Chart, WasmRenderer, component::Axis, element::{AxisType, Symbol}, series::Line
};
use leptos::prelude::*;
use types::HatSample;

#[component]
pub fn Temperature(sample: Signal<Option<HatSample>>) -> impl IntoView {
  let temp_class = move || {
    sample
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
  view! {
    <div class="flex flex-col">
      <div class="stat">
        <div class="stat-figure text-secondary">
          // Icon Nhiệt kế
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
            class="w-8 h-8"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              d="M12 9v3.75m9-.75a9 9 0 11-18 0 9 9 0 0118 0zm-9 3.75h.008v.008H12v-.008z"
            />
          </svg>
        </div>
        <div class="stat-title">"Nhiệt độ"</div>
        <div class=move || {
          format!("stat-value {}", temp_class())
        }>
          {move || match sample.get() {
            Some(s) => format!("{:.1}°C", s.temperature),
            None => "--".to_string(),
          }}
        </div>
        <div class="stat-desc">"Ngưỡng an toàn: 20-30°C"</div>
      </div>
    // <Graph sample=temp_and_ts />
    </div>
  }
}

#[component]
pub fn Graph(sample: Signal<Option<HatSample>>) -> impl IntoView {
  let data = RwSignal::new(VecDeque::with_capacity(20));
  Effect::new(move |_| {
    if let Some(sample) = sample.get() {
      data.update(|deq| {
        if deq.len() == 20 {
          deq.pop_front();
        }
        deq.push_back(sample);
      });
    }
  });

  Effect::new(move |_| {
    let data = data
      .get()
      .into_iter()
      .map(|sample| sample.temperature)
      .collect();
    let chart = Chart::new()
      .x_axis(Axis::new().type_(AxisType::Category))
      .y_axis(Axis::new().type_(AxisType::Value))
      .series(
        Line::new()
          .smooth(true)
          .symbol(Symbol::Circle)
          // .line_style(LineStyle::new().width(5).color("#5470C6"))
          // .area_style(AreaStyle::new())
          .data(data),
      );
    WasmRenderer::new(800, 400).render("temperature-chart", &chart).unwrap();
  });
  view! {
    // Container chính: Card giao diện, căn giữa, đổ bóng
    <div class="card w-full max-w-4xl bg-base-100 shadow-xl border border-base-200 mx-auto">
      <div class="card-body p-6">

        // --- Phần Header của Card ---
        <div class="flex flex-row justify-between items-center mb-4">
          // Tiêu đề + Icon
          <h2 class="card-title text-primary text-xl flex gap-2 items-center">
            // Icon Nhiệt kế (SVG)
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              stroke-width="1.5"
              stroke="currentColor"
              class="w-6 h-6"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M12 9v3.75m9-.75a9 9 0 11-18 0 9 9 0 0118 0zm-9 3.75h.008v.008H12v-.008z"
              />
            </svg>
            "Biểu đồ Nhiệt độ"
          </h2>

          // Badge trạng thái "Live" nhấp nháy
          <div class="badge badge-secondary badge-outline gap-2 animate-pulse">
            <div class="w-2 h-2 bg-secondary rounded-full"></div>
            "LIVE"
          </div>
        </div>

        // --- Phần chứa biểu đồ ---
        // w-full để chart co giãn theo card
        // h-[400px] để giữ chiều cao cố định, tránh nhảy layout khi load
        <div class="w-full h-400px rounded-lg overflow-hidden bg-base-100 relative">
          <div id="temperature-chart" class="w-full h-full"></div>
        </div>
      </div>
    </div>
  }
}
