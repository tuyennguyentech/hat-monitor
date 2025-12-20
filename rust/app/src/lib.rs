use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
  components::{Route, Router, Routes},
  StaticSegment,
};

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
  let count = RwSignal::new(0);
  let on_click = move |_| *count.write() += 1;

  view! {
    <div class="flex flex-col items-center justify-center h-screen bg-base-100 gap-8">
      <h1 class="text-5xl font-black bg-clip-text text-transparent bg-linear-to-r from-primary to-secondary">
        "Leptos Counter"
      </h1>

      <div class="stats shadow-lg bg-base-200 border border-base-300">
        <div class="stat place-items-center px-10 py-6">
          <div class="stat-title text-lg uppercase tracking-widest">"Total Clicks"</div>

          <div class="stat-value text-primary text-6xl my-2 font-mono">{count}</div>

          <div class="stat-desc">"Click to increase value"</div>
        </div>
      </div>

      // Nút bấm tách rời
      <button on:click=on_click class="btn btn-outline btn-primary btn-wide gap-2">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
          class="w-6 h-6"
        >
          <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
        </svg>
        "Increase Counter"
      </button>
    </div>
  }
}
