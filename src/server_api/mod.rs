use serde::{Deserialize, Serialize};

pub mod auth;
pub mod book;
pub mod progress;

pub mod authors;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageItems<T> {
    pub page: u64,
    pub max_item: u64,
    pub number_of_items: u64,
    pub number_of_pages: u64,
    pub items: Vec<T>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub role: i32,
}

#[cfg(feature = "ssr")]
pub mod ssr {

    pub use sea_orm::ModelTrait;

    pub use crate::entities::{self, prelude::*};
    pub use axum_session_auth::{Authentication, HasPermission, SessionSqlitePool};
    pub use sea_orm::ActiveModelTrait;
    pub use sea_orm::ColumnTrait;
    pub use sea_orm::EntityTrait;
    pub use sea_orm::QueryFilter;
    pub use sea_orm::SqlxSqliteConnector;
    pub use sqlx::SqlitePool;
    pub use std::collections::HashSet;
    pub type AuthSession = axum_session_auth::AuthSession<User, i32, SessionSqlitePool, SqlitePool>;
    pub use async_trait::async_trait;
    pub use bcrypt::{hash, verify, DEFAULT_COST};

    use super::User;
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
    #[derive(sqlx::FromRow, Clone)]
    pub struct SqlPermissionTokens {
        pub token: String,
    }
    impl From<entities::account::Model> for User {
        fn from(account: entities::account::Model) -> Self {
            User {
                id: account.id,
                username: account.name,
                role: account.role_level,
            }
        }
    }
    #[async_trait]
    impl Authentication<User, i32, SqlitePool> for User {
        async fn load_user(userid: i32, pool: Option<&SqlitePool>) -> Result<User, anyhow::Error> {
            let pool = pool.unwrap();
            let db = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone());

            use crate::entities::prelude::*;
            let user = Account::find_by_id(userid).one(&db).await?.unwrap();
            Ok(user.into())
        }

        fn is_authenticated(&self) -> bool {
            true
        }

        fn is_active(&self) -> bool {
            true
        }

        fn is_anonymous(&self) -> bool {
            false
        }
    }

    #[async_trait]
    impl HasPermission<SqlitePool> for User {
        async fn has(&self, perm: &str, _pool: &Option<&SqlitePool>) -> bool {
            match perm {
                "admin" => self.role == 0,
                "user" => self.role <= 1,
                _ => false,
            }
        }
    }
}
