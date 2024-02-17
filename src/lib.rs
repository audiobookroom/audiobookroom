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
    use ui::app::App;

    // _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
