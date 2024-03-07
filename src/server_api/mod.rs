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
    pub use axum_session_auth::{Authentication, HasPermission};

    #[cfg(feature = "mysql")]
    pub use axum_session_auth::SessionMySqlPool as SessionPool;
    #[cfg(feature = "sqlite")]
    pub use axum_session_auth::SessionSqlitePool as SessionPool;

    pub use sea_orm::ActiveModelTrait;
    pub use sea_orm::ColumnTrait;
    pub use sea_orm::EntityTrait;
    pub use sea_orm::QueryFilter;

    #[cfg(feature = "mysql")]
    pub use sea_orm::SqlxMySqlConnector as SqlxConnector;
    #[cfg(feature = "sqlite")]
    pub use sea_orm::SqlxSqliteConnector as SqlxConnector;
    #[cfg(feature = "mysql")]
    pub use sqlx::MySqlPool as SqlxPool;
    #[cfg(feature = "sqlite")]
    pub use sqlx::SqlitePool as SqlxPool;

    pub use std::collections::HashSet;
    pub type AuthSession = axum_session_auth::AuthSession<User, i32, SessionPool, SqlxPool>;
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
    impl Authentication<User, i32, SqlxPool> for User {
        async fn load_user(userid: i32, pool: Option<&SqlxPool>) -> Result<User, anyhow::Error> {
            let pool = pool.unwrap();
            #[cfg(feature = "sqlite")]
            let db = SqlxConnector::from_sqlx_sqlite_pool(pool.clone());
            #[cfg(feature = "mysql")]
            let db = SqlxConnector::from_sqlx_mysql_pool(pool.clone());

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
    impl HasPermission<SqlxPool> for User {
        async fn has(&self, perm: &str, _pool: &Option<&SqlxPool>) -> bool {
            match perm {
                "admin" => self.role == 0,
                "user" => self.role <= 1,
                _ => false,
            }
        }
    }
}
