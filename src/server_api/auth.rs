use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub role: i32,
}

#[cfg(feature = "ssr")]
pub mod ssr {

    pub use crate::entities::account;
    pub use crate::entities::prelude::*;
    pub use axum_session_auth::{Authentication, HasPermission, SessionSqlitePool};
    pub use sea_orm::ActiveModelTrait;
    pub use sea_orm::ColumnTrait;
    pub use sea_orm::EntityTrait;
    pub use sea_orm::QueryFilter;
    pub use sea_orm::SqlxSqliteConnector;
    pub use sqlx::SqlitePool;
    pub use std::collections::HashSet;
    pub type AuthSession = axum_session_auth::AuthSession<User, i32, SessionSqlitePool, SqlitePool>;
    pub use crate::ssr::{auth, db};
    pub use async_trait::async_trait;
    pub use bcrypt::{hash, verify, DEFAULT_COST};

    use super::User;

    #[derive(sqlx::FromRow, Clone)]
    pub struct SqlPermissionTokens {
        pub token: String,
    }

    #[async_trait]
    impl Authentication<User, i32, SqlitePool> for User {
        async fn load_user(userid: i32, pool: Option<&SqlitePool>) -> Result<User, anyhow::Error> {
            let pool = pool.unwrap();
            let db = SqlxSqliteConnector::from_sqlx_sqlite_pool(pool.clone());

            use crate::entities::prelude::*;
            let user = Account::find_by_id(userid).one(&db).await?.unwrap();
            Ok(User {
                id: user.id,
                username: user.name,
                role: user.role_level,
            })
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

#[server]
/// get current login user
pub async fn get_user() -> Result<Option<User>, ServerFnError> {
    use crate::ssr::auth;
    let auth = auth()?;
    Ok(auth.current_user)
}

#[server]
pub async fn have_user() -> Result<bool, ServerFnError> {
    use self::ssr::*;
    let db = db()?;

    let user = crate::entities::account::Entity::find().all(&db).await?;
    Ok(!user.is_empty())
}

#[server(Login, "/api")]
pub async fn login(
    username: String,
    password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    use self::ssr::*;
    let db = db()?;
    let auth = auth()?;

    let user = crate::entities::account::Entity::find()
        .filter(account::Column::Name.eq(username))
        .one(&db)
        .await?
        .ok_or(ServerFnError::new("Login failed: User does not exist."))?;

    match verify(password, &user.password)? {
        true => {
            auth.login_user(user.id);
            auth.remember_user(remember.is_some());
            leptos_axum::redirect("/");
            Ok(())
        }
        false => Err(ServerFnError::ServerError(
            "Password does not match.".to_string(),
        )),
    }
}

#[server(Signup, "/api")]
pub async fn signup(
    username: String,
    password: String,
    remember: Option<String>,
) -> Result<(), ServerFnError> {
    use self::ssr::*;

    let db = db()?;
    let auth = auth()?;

    let password_hashed = hash(password, DEFAULT_COST).unwrap();

    let user = account::ActiveModel {
        name: sea_orm::ActiveValue::Set(username),
        password: sea_orm::ActiveValue::Set(password_hashed),
        role_level: sea_orm::ActiveValue::Set(1),
        ..Default::default()
    };
    let user = user.insert(&db).await?;

    auth.login_user(user.id);
    auth.remember_user(remember.is_some());

    leptos_axum::redirect("/");

    Ok(())
}

#[server(Logout, "/api")]
pub async fn logout() -> Result<(), ServerFnError> {
    use self::ssr::*;

    let auth = auth()?;

    auth.logout_user();
    leptos_axum::redirect("/");

    Ok(())
}
