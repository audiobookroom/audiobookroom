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
#[cfg(feature = "ssr")]
pub mod ssr {
    use crate::server_api::auth::ssr::AuthSession;
    use leptos::*;
    use sea_orm::DatabaseConnection;

    pub fn db() -> Result<DatabaseConnection, ServerFnError> {
        use_context::<DatabaseConnection>()
            .ok_or_else(|| ServerFnError::ServerError("DatabaseConnection missing.".into()))
    }

    pub fn auth() -> Result<AuthSession, ServerFnError> {
        use_context::<AuthSession>()
            .ok_or_else(|| ServerFnError::ServerError("Auth session missing.".into()))
    }

    pub fn init_logger_info() {
        tracing_subscriber::fmt::SubscriberBuilder::default()
            .with_env_filter(
                tracing_subscriber::EnvFilter::builder()
                    .with_default_directive(tracing::level_filters::LevelFilter::INFO.into())
                    .from_env_lossy(),
            )
            .init();
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use ui::app::App;

    // _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
