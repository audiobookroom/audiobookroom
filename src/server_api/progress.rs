use leptos::*;
use serde::{Deserialize, Serialize};

use crate::ProgressDateType;

use super::book::{BookDetail, ChapterDetail};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProgressResult {
    pub account_id: i32,
    pub music_id: i32,
    pub chapter_id: i32,
    pub progress: f64,

    pub update: ProgressDateType,
}

#[cfg(feature = "ssr")]
impl From<crate::entities::progress::Model> for ProgressResult {
    fn from(p: crate::entities::progress::Model) -> Self {
        Self {
            account_id: p.account_id,
            music_id: p.music_id,
            chapter_id: p.chapter_id,
            progress: p.progress,
            update: p.update,
        }
    }
}

#[server]
pub async fn get_progress_detail_by_user(
    account_id: i32,
) -> Result<Vec<(ProgressResult, BookDetail, ChapterDetail)>, ServerFnError> {
    crate::server_api::auth::get_user()
        .await?
        .ok_or(ServerFnError::new("Not logged in"))?;

    use super::ssr::*;
    use crate::entities::*;
    use sea_orm::prelude::*;
    let db = db()?;

    let p = Progress::find()
        .filter(progress::Column::AccountId.eq(account_id))
        .all(&db)
        .await?;
    let book = p
        .load_one(Music, &db)
        .await?
        .into_iter()
        .map(Option::unwrap);
    let chapter = p
        .load_one(Chapter, &db)
        .await?
        .into_iter()
        .map(Option::unwrap);
    let p = p
        .into_iter()
        .zip(book)
        .zip(chapter)
        .map(|((p, b), c)| (p.into(), b.into(), c.into()))
        .collect::<Vec<_>>();
    Ok(p)
}

#[server]
pub async fn get_progress_by_user(account_id: i32) -> Result<Vec<ProgressResult>, ServerFnError> {
    crate::server_api::auth::get_user()
        .await?
        .ok_or(ServerFnError::new("Not logged in"))?;

    use super::ssr::*;
    use crate::entities::*;
    let db = db()?;

    let p = Progress::find()
        .filter(progress::Column::AccountId.eq(account_id))
        .all(&db)
        .await?;
    tracing::info!("get progress: {:?}", p);
    let p = p.into_iter().map(Into::into).collect();

    Ok(p)
}
#[server]
pub async fn get_progress(
    music_id: i32,
    account_id: i32,
) -> Result<Option<ProgressResult>, ServerFnError> {
    crate::server_api::auth::get_user()
        .await?
        .ok_or(ServerFnError::new("Not logged in"))?;

    use super::ssr::*;
    let db = db()?;

    let p = Progress::find_by_id((account_id, music_id))
        .one(&db)
        .await?;
    tracing::info!("get progress: {:?}", p);
    let p = p.map(Into::into);

    Ok(p)
}

#[server]
pub async fn set_progress(
    account_id: i32,
    music_id: i32,
    chapter_id: i32,
    progress: f64,
) -> Result<(), ServerFnError> {
    crate::server_api::auth::get_user()
        .await?
        .ok_or(ServerFnError::new("Not logged in"))?;

    use super::ssr::*;
    use crate::entities::*;
    let db = db()?;
    let auth = auth()?;
    let current_user = auth.current_user;
    let current_user = current_user.ok_or(ServerFnError::new("Not logged in"))?;
    if current_user.id != account_id {
        return Err(ServerFnError::new("Not authorized"));
    }
    let p = Progress::find_by_id((account_id, music_id))
        .one(&db)
        .await?;
    if let Some(p) = p {
        use sea_orm::IntoActiveModel;

        let mut p = p.into_active_model();
        p.chapter_id = sea_orm::ActiveValue::set(chapter_id);
        p.progress = sea_orm::ActiveValue::set(progress);
        let now = chrono::Utc::now();
        #[cfg(feature = "sqlite")]
        let now = now.to_rfc3339();
        p.update = sea_orm::ActiveValue::set(now);
        p.save(&db).await?;
    } else {
        let now = chrono::Utc::now();
        #[cfg(feature = "sqlite")]
        let now = now.to_rfc3339();
        Progress::insert(progress::ActiveModel {
            account_id: sea_orm::ActiveValue::set(account_id),
            music_id: sea_orm::ActiveValue::set(music_id),
            chapter_id: sea_orm::ActiveValue::set(chapter_id),
            progress: sea_orm::ActiveValue::set(progress),
            update: sea_orm::ActiveValue::set(now),
        })
        .exec(&db)
        .await?;
    }

    Ok(())
}
