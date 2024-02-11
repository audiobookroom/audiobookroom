use axum::extract::FromRef;
use leptos::LeptosOptions;
use leptos_router::RouteListing;
use sea_orm::DatabaseConnection;

/// This takes advantage of Axum's SubStates feature by deriving FromRef. This is the only way to have more than one
/// item in Axum's State. Leptos requires you to have leptosOptions in your State struct for the leptos route handlers
#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub routes: Vec<RouteListing>,
    pub db: DatabaseConnection,
}
