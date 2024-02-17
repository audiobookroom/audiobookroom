use super::User;
use leptos::*;
#[server]
/// get current login user
pub async fn get_user() -> Result<Option<User>, ServerFnError> {
    use super::ssr::*;
    let auth = auth()?;
    Ok(auth.current_user)
}
#[server]
pub async fn get_all_users()->Result<Vec<User>, ServerFnError>{
    use super::ssr::*;
    let db = db()?;
    let users = crate::entities::account::Entity::find().all(&db).await?;
    let users = users.into_iter().map(Into::into).collect();
    Ok(users)
}
#[server]
pub async fn is_admin() -> Result<bool, ServerFnError> {
    let user = get_user().await?;
    let is_admin = user.is_some_and(|user| user.role == 0);
    Ok(is_admin)
}

#[server]
pub async fn have_user() -> Result<bool, ServerFnError> {
    use super::ssr::*;
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
    use super::ssr::*;
    let db = db()?;
    let auth = auth()?;

    let user = entities::account::Entity::find()
        .filter(entities::account::Column::Name.eq(username))
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
    use super::ssr::*;
    let have_user = have_user().await?;
    if have_user {
        return Err(ServerFnError::ServerError(
            "initial signup can only happen at the very first access".to_string(),
        ));
    }
    let auth = auth()?;
    let user = add_user_util(username, password, 0).await?;
    auth.login_user(user.id);
    auth.remember_user(remember.is_some());
    leptos_axum::redirect("/");
    Ok(())
}
#[server]
pub async fn add_user(username: String, password: String, role: i32) -> Result<(), ServerFnError> {
    let is_admin = is_admin().await?;
    if !is_admin {
        return Err(ServerFnError::ServerError(
            "You are not authorized to add user.".to_string(),
        ));
    }

    add_user_util(username, password, role).await?;

    Ok(())
}
#[server]
pub async fn change_user_passwd(user_id: i32, new_password: String) -> Result<(), ServerFnError> {
    use super::ssr::*;
    let db = db()?;
    let user = get_user().await?;
    let is_current_user_or_admin = user.is_some_and(|user| user.role == 0 || user.id == user_id);
    if !is_current_user_or_admin {
        return Err(ServerFnError::ServerError(
            "You are not authorized to change user password.".to_string(),
        ));
    }
    let password_hashed = hash(new_password, DEFAULT_COST).unwrap();
    let user = entities::account::ActiveModel {
        id: sea_orm::ActiveValue::Set(user_id),
        password: sea_orm::ActiveValue::Set(password_hashed),
        ..Default::default()
    };
    user.update(&db).await?;
    Ok(())
}

#[server]
pub async fn delete_user(user_id: i32) -> Result<(), ServerFnError> {
    use super::ssr::*;
    let db = db()?;
    let is_admin = is_admin().await?;
    if !is_admin {
        return Err(ServerFnError::ServerError(
            "You are not authorized to delete user.".to_string(),
        ));
    }
    // before delete the user, should delete all the progress of the user
    let account = Account::find_by_id(user_id).one(&db).await?;

    let account = account.ok_or(ServerFnError::new("User does not exist."))?;
    let progresses = account.find_related(Progress).all(&db).await?;
    for progress in progresses {
        progress.delete(&db).await?;
    }
    account.delete(&db).await?;

    Ok(())
}
#[cfg(feature = "ssr")]
pub async fn add_user_util(
    username: String,
    password: String,
    role: i32,
) -> Result<User, ServerFnError> {
    use super::ssr::*;
    let db = db()?;

    let password_hashed = hash(password, DEFAULT_COST).unwrap();

    let user = entities::account::ActiveModel {
        name: sea_orm::ActiveValue::Set(username),
        password: sea_orm::ActiveValue::Set(password_hashed),
        role_level: sea_orm::ActiveValue::Set(role),
        ..Default::default()
    };

    let user = user.insert(&db).await?;

    Ok(user.into())
}
#[server(Logout, "/api")]
pub async fn logout() -> Result<(), ServerFnError> {
    use super::ssr::*;
    let auth = auth()?;

    auth.logout_user();
    leptos_axum::redirect("/");

    Ok(())
}
