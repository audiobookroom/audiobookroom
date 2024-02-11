use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};

use super::PageItems;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthorDetail {
    pub id: i32,
    pub avatar: String,
    pub name: String,
    pub description: String,
}
#[cfg(feature = "ssr")]
impl From<crate::entities::author::Model> for AuthorDetail {
    fn from(a: crate::entities::author::Model) -> Self {
        Self {
            id: a.id,
            avatar: a.avatar,
            name: a.name,
            description: a.description,
        }
    }
}

#[server]
pub async fn get_author_by_id(id: i32) -> Result<Option<AuthorDetail>, ServerFnError> {
    use crate::entities::prelude::*;
    use crate::ssr::*;
    use sea_orm::prelude::*;
    let db = db()?;
    crate::server_api::auth::get_user()
        .await?
        .ok_or(ServerFnError::new("Not logged in"))?;
    let author = Author::find_by_id(id).one(&db).await?;
    let author = author.map(Into::into);
    Ok(author)
}

#[server]
pub async fn list_all_authors(
    page_num: u64,
    max_item: u64,
) -> Result<PageItems<AuthorDetail>, ServerFnError> {
    use crate::entities::author;
    use crate::entities::prelude::*;
    use crate::ssr::*;
    use sea_orm::prelude::*;
    use sea_orm::*;
    let db = db()?;
    crate::server_api::auth::get_user()
        .await?
        .ok_or(ServerFnError::new("Not logged in"))?;
    let page = Author::find()
        .order_by_asc(author::Column::Id)
        .paginate(&db, max_item);
    let ItemsAndPagesNumber {
        number_of_items,
        number_of_pages,
    } = page.num_items_and_pages().await?;
    let item = page.fetch_page(page_num).await?;
    let authors = item.into_iter().map(Into::into).collect();
    let authors = PageItems {
        page: page_num,
        max_item,
        number_of_items,
        number_of_pages,
        items: authors,
    };
    Ok(authors)
}
