use charming::{
  component::{Axis, Title},
  element::AxisType,
  series::Line,
  Chart, WasmRenderer,
};
use leptos::prelude::*;
use leptos_use::{
  use_interval_fn_with_options, utils::Pausable, UseIntervalFnOptions,
};

#[component]
pub fn Graph() -> impl IntoView {
  let data = RwSignal::new(vec![150, 230, 224, 218, 135, 147, 260]);
  Effect::new(move |_| {
    let local = data.get();

    let chart = Chart::new()
      .title(Title::new().text("Demo: Leptos + Charming"))
      .x_axis(
        Axis::new()
          .type_(AxisType::Category)
          .data(vec!["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]),
      )
      .y_axis(Axis::new().type_(AxisType::Value))
      .series(Line::new().data(local));

    let renderer = WasmRenderer::new(600, 400);
    renderer.render("chart", &chart).unwrap();
  });

  let Pausable {
    pause,
    resume,
    is_active: _,
  } = use_interval_fn_with_options(
    move || {
      data.update(|d| d.rotate_right(1));
    },
    1000,
    UseIntervalFnOptions::default().immediate(false),
  );
  let resume_clone = resume.clone();
  Effect::new(move |_| {
    resume_clone();
  });
  view! {
    <div>
      <div id="chart"></div>
      <button class="btn" on:click=move |_| pause()>
        "Pause"
      </button>
      <button class="btn" on:click=move |_| resume()>
        "Resume"
      </button>
    </div>
  }
}
