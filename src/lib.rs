#![feature(let_chains)]
pub mod error_template;
pub mod errors;
pub mod server_api;
pub mod ui;

#[cfg(feature = "ssr")]
pub mod entities;
#[cfg(feature = "ssr")]
pub mod fallback;
#[cfg(feature = "ssr")]
pub mod middleware;
#[cfg(feature = "ssr")]
pub mod state;

#[cfg(feature = "ssr")]
pub mod tools;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use tracing_subscriber::fmt::format::Pretty;
    use tracing_subscriber::prelude::*;
    use tracing_web::{performance_layer, MakeWebConsoleWriter};
    use ui::app::App;
    use tracing_subscriber::fmt::time::UtcTime;
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false) // Only partially supported across browsers
        .with_timer(UtcTime::rfc_3339())
        .with_writer(MakeWebConsoleWriter::new()); // write events to the console
    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .init(); // Install these as subscribers to tracing events

    // _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
