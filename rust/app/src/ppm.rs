use std::collections::VecDeque;

use charming::{
  component::Axis,
  element::{AxisType, Symbol},
  series::Line,
  Chart, WasmRenderer,
};
use leptos::prelude::*;
use types::HatSample;

#[component]
pub fn Ppm(sample: Signal<Option<HatSample>>) -> impl IntoView {
  let ppm_class = move || {
    sample
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
  view! {
    <div class="stat">
      <div class="stat-figure text-accent">
        // Icon Đám mây/Gas
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
            d="M12 16.5V9.75m0 0l3 3m-3-3l-3 3M6.75 19.5a4.5 4.5 0 01-1.41-8.775 5.25 5.25 0 0110.233-2.33 3 3 0 013.758 3.848A3.752 3.752 0 0118 19.5H6.75z"
          />
        </svg>
      </div>
      <div class="stat-title">"Chất lượng khí (CO2)"</div>
      <div class=move || {
        format!("stat-value {}", ppm_class())
      }>
        {move || match sample.get() {
          Some(s) => format!("{:.0} PPM", s.corrected_ppm),
          None => "--".to_string(),
        }}
      </div>
      // Hiển thị thêm thông số kỹ thuật (Resistance) ở phần mô tả
      <div class="stat-desc text-xs">
        {move || match sample.get() {
          Some(s) => format!("R: {:.0} Ω | R0: {:.0} Ω", s.resistance, s.corrected_r_zero),
          None => "Đang hiệu chuẩn...".to_string(),
        }}
      </div>
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
      .map(|sample| sample.corrected_ppm)
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
    WasmRenderer::new(800, 400).render("ppm-chart", &chart).unwrap();
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
            "Biểu đồ PPM"
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
          <div id="ppm-chart" class="w-full h-full"></div>
        </div>
      </div>
    </div>
  }
}