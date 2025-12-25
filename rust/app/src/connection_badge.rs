use leptos::prelude::*;
use leptos_use::core::ConnectionReadyState;

#[component]
pub fn ConnectionBadge(ready: Signal<ConnectionReadyState>) -> impl IntoView {
  println!("{:#?}", ready);
  view! {
    {move || match ready.get() {
      ConnectionReadyState::Open => view! { <div class="badge badge-success gap-2">"Online"</div> },
      ConnectionReadyState::Closed => view! { <div class="badge badge-error gap-2">"Offline"</div> },
      _ => view! { <div class="badge badge-warning gap-2">"Connecting..."</div> },
    }}
  }
}
