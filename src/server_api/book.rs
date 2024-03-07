use leptos::*;
use serde::{Deserialize, Serialize};

use super::authors::AuthorDetail;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BookChapter {
    chapter_id: i32,
    book_id: i32,
    name: String,
    url: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageItems<T> {
    pub page: u64,
    pub max_item: u64,
    pub number_of_items: u64,
    pub number_of_pages: u64,
    pub items: Vec<T>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BookDetail {
    pub id: i32,
    pub author_id: i32,
    pub name: String,
    pub chapters: i32,
    pub total_time: Option<f64>,
    pub file_folder: String,
    pub music_type: i32,
}

#[cfg(feature = "ssr")]
impl From<crate::entities::music::Model> for BookDetail {
    fn from(m: crate::entities::music::Model) -> Self {
        Self {
            id: m.id,
            author_id: m.author_id,
            name: m.name,
            chapters: m.chapters,
            total_time: m.total_time,
            file_folder: m.file_folder,
            music_type: m.music_type,
        }
    }
}

#[server]
pub async fn get_books_by_author(
    author_id: i32,
    page_num: u64,
    max_item: u64,
) -> Result<PageItems<BookDetail>, ServerFnError> {
    crate::server_api::auth::get_user()
        .await?
        .ok_or(ServerFnError::new("Not logged in"))?;

    use crate::entities::{prelude::*, *};

    use super::ssr::*;
    use sea_orm::prelude::*;
    use sea_orm::{ItemsAndPagesNumber, QueryOrder};
    let db = db()?;
    let page = Music::find()
        .filter(music::Column::AuthorId.eq(author_id))
        .order_by_asc(music::Column::Id)
        .paginate(&db, max_item);
    let ItemsAndPagesNumber {
        number_of_items,
        number_of_pages,
    } = page.num_items_and_pages().await?;
    let item = page.fetch_page(page_num).await?;
    let books = item
        .into_iter()
        .map(|it| BookDetail {
            id: it.id,
            author_id: it.author_id,
            name: it.name,
            chapters: it.chapters,
            total_time: it.total_time,
            file_folder: it.file_folder,
            music_type: it.music_type,
        })
        .collect();
    Ok(PageItems {
        number_of_items,
        number_of_pages,
        page: page_num,
        max_item: max_item,
        items: books,
    })
}

#[server(GetBookAllDetail,"/api","GetJson")]
pub async fn get_book_all_detail(
    book_id: i32,
    current_page: u64,
    max_item: u64,
) -> Result<
    (
        BookDetail,
        crate::server_api::authors::AuthorDetail,
        ChapterPage,
        Option<(super::progress::ProgressResult, ChapterDetail)>,
    ),
    ServerFnError,
> {
    use super::ssr::*;
    use crate::entities::*;
    let db = db()?;
    let user = crate::server_api::auth::get_user()
        .await?
        .ok_or(ServerFnError::new("Not logged in"))?;
    let (book, author) = Music::find_by_id(book_id)
        .find_also_related(author::Entity)
        .one(&db)
        .await?
        .unwrap();
    use sea_orm::PaginatorTrait;

    let pages = Chapter::find()
        .filter(chapter::Column::MusicId.eq(book_id))
        .paginate(&db, max_item);
    let page = pages.fetch_page(current_page).await?;
    let progres = Progress::find_by_id((user.id, book_id))
        .find_also_related(chapter::Entity)
        .one(&db)
        .await?;
    Ok((
        book.into(),
        author.unwrap().into(),
        ChapterPage {
            page: current_page as u64,
            max_item: max_item as u64,
            number_of_items: pages.num_items().await?,
            number_of_pages: pages.num_pages().await?,
            items: page.into_iter().map(Into::into).collect(),
        },
        progres.map(|(p, c)| (p.into(), c.unwrap().into())),
    ))
}
#[server]
pub async fn get_book_detail(book_id: i32) -> Result<BookDetail, ServerFnError> {
    crate::server_api::auth::get_user()
        .await?
        .ok_or(ServerFnError::new("Not logged in"))?;

    use super::ssr::*;
    let db = db()?;
    let book = Music::find_by_id(book_id)
        .one(&db)
        .await?
        .ok_or(ServerFnError::new("Book not found"))?;
    Ok(book.into())
}

#[server]
pub async fn get_books_details(
    page_num: u64,
    max_item: u64,
) -> Result<PageItems<(BookDetail, AuthorDetail)>, ServerFnError> {
    crate::server_api::auth::get_user()
        .await?
        .ok_or(ServerFnError::new("Not logged in"))?;

    use crate::entities::{prelude::*, *};

    use super::ssr::*;
    use sea_orm::prelude::*;
    use sea_orm::{ItemsAndPagesNumber, QueryOrder};
    let db = db()?;
    let page = Music::find()
        .order_by_asc(music::Column::Id)
        .paginate(&db, max_item);
    let ItemsAndPagesNumber {
        number_of_items,
        number_of_pages,
    } = page.num_items_and_pages().await?;
    let item = page.fetch_page(page_num).await?;
    let authors = item.load_one(Author, &db).await?;

    let items = item
        .into_iter()
        .zip(authors)
        .map(|(it, au)| (it.into(), au.unwrap().into()))
        .collect();

    Ok(PageItems {
        number_of_items,
        number_of_pages,
        page: page_num,
        max_item: max_item,
        items,
    })
}

#[server]
pub async fn get_books(
    page_num: u64,
    max_item: u64,
) -> Result<PageItems<BookDetail>, ServerFnError> {
    crate::server_api::auth::get_user()
        .await?
        .ok_or(ServerFnError::new("Not logged in"))?;

    use crate::entities::{prelude::*, *};

    use super::ssr::*;
    use sea_orm::prelude::*;
    use sea_orm::{ItemsAndPagesNumber, QueryOrder};
    let db = db()?;
    let page = Music::find()
        .order_by_asc(music::Column::Id)
        .paginate(&db, max_item);
    let ItemsAndPagesNumber {
        number_of_items,
        number_of_pages,
    } = page.num_items_and_pages().await?;
    let item = page.fetch_page(page_num).await?;
    let books = item
        .into_iter()
        .map(|it| BookDetail {
            id: it.id,
            author_id: it.author_id,
            name: it.name,
            chapters: it.chapters,
            total_time: it.total_time,
            file_folder: it.file_folder,
            music_type: it.music_type,
        })
        .collect();
    Ok(PageItems {
        number_of_items,
        number_of_pages,
        page: page_num,
        max_item: max_item,
        items: books,
    })
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddBookResult {
    pub msg: String,
}

#[server]
pub async fn add_book(
    author_name: String,
    name: String,
    source: String,
) -> Result<AddBookResult, ServerFnError> {
    crate::server_api::auth::get_user()
        .await?
        .ok_or(ServerFnError::new("Not logged in"))?;

    use super::ssr::*;
    let db = db()?;
    // first create the author

    let create_result = crate::tools::create_new_book(
        author_name,
        name,
        std::path::Path::new("./fetchbook"),
        std::path::Path::new(&source),
        &db,
    )
    .await;
    if let Err(e) = create_result {
        return Err(ServerFnError::new(e.to_string()));
    }

    Ok(AddBookResult {
        msg: "Book added".to_string(),
    })
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChapterPage {
    pub page: u64,
    pub max_item: u64,
    pub number_of_items: u64,
    pub number_of_pages: u64,
    pub items: Vec<ChapterDetail>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChapterDetail {
    pub id: i32,
    pub music_id: i32,
    pub chapter_num: i32,
    pub chapter_name: String,
    pub chapter_url: String,
    pub chapter_length: Option<f64>,
}

#[cfg(feature = "ssr")]
impl From<crate::entities::chapter::Model> for ChapterDetail {
    fn from(c: crate::entities::chapter::Model) -> Self {
        Self {
            id: c.id,
            music_id: c.music_id,
            chapter_num: c.chapter_num,
            chapter_name: c.chapter_name,
            chapter_url: c.chapter_url,
            chapter_length: c.chapter_length,
        }
    }
}

#[server(GetChapters,"/api","GetJson")]
pub async fn get_chapters(
    music_id: i32,
    page_num: u64,
    max_item: u64,
) -> Result<ChapterPage, ServerFnError> {
    crate::server_api::auth::get_user()
        .await?
        .ok_or(ServerFnError::new("Not logged in"))?;

    use super::ssr::*;
    use crate::entities::*;

    use sea_orm::prelude::*;
    use sea_orm::{ItemsAndPagesNumber, QueryOrder};
    let db = db()?;
    let page = Chapter::find()
        .filter(chapter::Column::MusicId.eq(music_id))
        .order_by_asc(chapter::Column::ChapterNum)
        .paginate(&db, max_item);
    let ItemsAndPagesNumber {
        number_of_items,
        number_of_pages,
    } = page.num_items_and_pages().await?;
    let item = page.fetch_page(page_num).await?;
    let chapters = item
        .into_iter()
        .map(|it| ChapterDetail {
            id: it.id,
            music_id: it.music_id,
            chapter_num: it.chapter_num,
            chapter_name: it.chapter_name,
            chapter_url: it.chapter_url,
            chapter_length: it.chapter_length,
        })
        .collect();
    Ok(ChapterPage {
        number_of_items,
        number_of_pages,
        page: page_num,
        max_item: max_item,
        items: chapters,
    })
}

#[server]
pub async fn search_chapter_by_chapter_num(
    book_id: i32,
    chapter_num: i32,
) -> Result<ChapterDetail, ServerFnError> {
    crate::server_api::auth::get_user()
        .await?
        .ok_or(ServerFnError::new("Not logged in"))?;
    use super::ssr::*;

    let db = db()?;
    use crate::entities::chapter;
    let chapter = Chapter::find()
        .filter(chapter::Column::MusicId.eq(book_id))
        .filter(chapter::Column::ChapterNum.eq(chapter_num))
        .one(&db)
        .await?
        .ok_or(ServerFnError::new("Chapter not found"))?;
    Ok(ChapterDetail {
        id: chapter.id,
        music_id: chapter.music_id,
        chapter_num: chapter.chapter_num,
        chapter_name: chapter.chapter_name,
        chapter_url: chapter.chapter_url,
        chapter_length: chapter.chapter_length,
    })
}

#[server]
pub async fn get_chapter_details(
    chapter_id: i32,
) -> Result<(BookDetail, AuthorDetail, ChapterDetail), ServerFnError> {
    use super::ssr::*;
    use crate::entities::*;

    let db = db()?;
    let _user = crate::server_api::auth::get_user()
        .await?
        .ok_or(ServerFnError::new("Not logged in"))?;
    let (chapter, book) = Chapter::find_by_id(chapter_id)
        .find_also_related(music::Entity)
        .one(&db)
        .await?
        .unwrap();
    let book = book.unwrap();
    let author = book.find_related(Author).one(&db).await?.unwrap();
    Ok((book.into(), author.into(), chapter.into()))
}
#[server]
pub async fn get_chatper_detail(chapter_id: i32) -> Result<ChapterDetail, ServerFnError> {
    crate::server_api::auth::get_user()
        .await?
        .ok_or(ServerFnError::new("Not logged in"))?;
    use super::ssr::*;

    let db = db()?;
    let chapter = Chapter::find_by_id(chapter_id)
        .one(&db)
        .await?
        .ok_or(ServerFnError::new("Chapter not found"))?;
    Ok(ChapterDetail {
        id: chapter.id,
        music_id: chapter.music_id,
        chapter_num: chapter.chapter_num,
        chapter_name: chapter.chapter_name,
        chapter_url: chapter.chapter_url,
        chapter_length: chapter.chapter_length,
    })
}

#[server]
pub async fn delete_book(book_id: i32) -> Result<(), ServerFnError> {
    crate::server_api::auth::get_user()
        .await?
        .ok_or(ServerFnError::new("Not logged in"))?;
    use super::ssr::*;

    use sea_orm::prelude::*;
    let db = db()?;
    use crate::entities::*;
    let book = Music::find_by_id(book_id).one(&db).await?;
    if let Some(book) = book {
        // delete the progress
        let all_progress = book.find_related(Progress).all(&db).await?;
        for p in all_progress {
            p.delete(&db).await?;
        }
        // delete the chapters
        let all_chapters = book.find_related(Chapter).all(&db).await?;
        for c in all_chapters {
            c.delete(&db).await?;
        }
        // delete the book
        Music::delete_by_id(book.id).exec(&db).await?;

        let book_count = Music::find()
            .filter(music::Column::AuthorId.eq(book.author_id))
            .count(&db)
            .await?;
        if book_count == 0 {
            // delete the author
            Author::delete_by_id(book.author_id).exec(&db).await?;
        }
    }
    Ok(())
}
