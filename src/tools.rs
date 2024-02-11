use std::path::{Path, PathBuf};

use crate::entities::{prelude::*, *};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tracing::{debug, info};
pub async fn arrange_new_folder(
    src_dir: impl AsRef<Path>,
    book_dir: impl AsRef<Path>,
    author: &str,
    book_name: &str,
) -> (Vec<PathBuf>, Vec<PathBuf>) {
    // book_dir : /fetchbook
    // src_dir /my_book_download
    // patrial_target_dir: author/book_name

    let book_dir = book_dir.as_ref();

    let patrial_target_dir = Path::new(author).join(book_name);

    let target_dir = book_dir.join(&patrial_target_dir);
    debug!("moving {:?} target_dir: {:?}", src_dir.as_ref(), target_dir);

    let files = get_files_in_dir(src_dir);
    // create target dir if not exists
    std::fs::create_dir_all(&target_dir).unwrap();

    // target_file_names : author/book_name/0001.ext
    let partial_target_file_names = files
        .iter()
        .zip(1..)
        .map(|(src, target_index)| {
            // create a hard link from src to target_dir, with new filename target_index+src.ext
            let target = patrial_target_dir.join(format!(
                "{:04}.{}",
                target_index,
                src.extension().unwrap().to_str().unwrap()
            ));
            target
        })
        .collect::<Vec<_>>();
    let target_file_paths = partial_target_file_names.iter().map(|f| book_dir.join(f));

    for (src, target) in files.iter().zip(target_file_paths) {
        // create a hard link from src to target_dir, with new filename target_index+src.ext
        info!("moving {:?} to {:?}", src, target);
        tokio::fs::hard_link(src, target).await.unwrap();
    }
    info!("files from src to {:?}", files);
    info!("files to {:?}", partial_target_file_names);
    (files, partial_target_file_names)
}
fn sort_with_number(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let fist_numer_reg = regex::Regex::new(r"\d+").unwrap();

    let mut out = paths
        .into_iter()
        .filter_map(|file| {
            let fd = fist_numer_reg.find(file.file_name().unwrap().to_str().unwrap());
            match fd {
                Some(m) => {
                    let num = m.as_str().parse::<u32>().unwrap();
                    Some((num, file))
                }
                None => None,
            }
        })
        .collect::<Vec<_>>();
    out.sort_by_key(|(num, _)| *num);
    out.into_iter().map(|(_, file)| file).collect()
}
fn get_files_in_dir(dir: impl AsRef<Path>) -> Vec<PathBuf> {
    let files_and_dirs = std::fs::read_dir(dir).unwrap();
    let entries: Vec<_> = files_and_dirs.into_iter().map(|e| e.unwrap()).collect();
    let mut files = vec![];
    let mut dirs = vec![];
    for e in entries {
        if e.file_type().unwrap().is_dir() {
            dirs.push(e.path());
        } else {
            files.push(e.path());
        }
    }
    let files_sorted = sort_with_number(files);
    let dirs_sorted = sort_with_number(dirs);
    let mut out = vec![];
    out.extend(files_sorted);
    for dir in dirs_sorted {
        out.extend(get_files_in_dir(&dir));
    }
    out
}

pub async fn create_new_book(
    author_name: String,
    new_book_name: String,
    book_dir: &Path,
    source_dir: &Path,
    db: &sea_orm::DatabaseConnection,
) -> eyre::Result<()> {
    let db_book_dir = format!("{}/{}", author_name, new_book_name);
    info!("book dir:{:?}", book_dir);

    // let target_dir = format!("{:?}/{}/{}", book_dir, author_name, new_book_name);
    // let target_dir = book_dir.join(&author_name).join(&new_book_name);
    let (srcs, targets) =
        arrange_new_folder(source_dir, book_dir, &author_name, &new_book_name).await;
    // create the book in db
    // first create the author
    let current_author = Author::find()
        .filter(author::Column::Name.eq(&author_name))
        .one(db)
        .await?;
    // if it's none, insert a new one
    let author_id = match current_author {
        Some(author) => author.id,
        None => {
            let author = Author::insert(author::ActiveModel {
                name: sea_orm::ActiveValue::Set(author_name),
                avatar: sea_orm::ActiveValue::Set("".to_string()),
                description: sea_orm::ActiveValue::Set("".to_string()),

                ..Default::default()
            })
            .exec(db)
            .await?;
            author.last_insert_id
        }
    };
    let mut extension = srcs.iter().filter_map(|f| f.extension());
    let next = extension.next().unwrap().to_str().unwrap();
    let music_type = match next {
        "mp3" => 0,
        "m4a" => 1,
        _ => panic!("unsupported file type"),
    };

    // insert the book
    let book = Music::insert(music::ActiveModel {
        name: sea_orm::ActiveValue::Set(new_book_name),
        author_id: sea_orm::ActiveValue::Set(author_id),
        chapters: sea_orm::ActiveValue::Set(srcs.len() as i32),
        file_folder: sea_orm::ActiveValue::Set(db_book_dir.clone()),
        music_type: sea_orm::ActiveValue::Set(music_type),
        ..Default::default()
    })
    .exec(db)
    .await?;
    let book_id = book.last_insert_id;
    info!("book created:{}", book_id);
    info!("book dir:{}", db_book_dir);
    let models = srcs
        .into_iter()
        .zip(targets)
        .enumerate()
        .map(|(i, (src, target))| chapter::ActiveModel {
            music_id: sea_orm::ActiveValue::Set(book_id),
            chapter_num: sea_orm::ActiveValue::Set(i as i32),
            chapter_name: sea_orm::ActiveValue::Set(
                src.file_stem().unwrap().to_string_lossy().to_string(),
            ),
            chapter_url: sea_orm::ActiveValue::Set(target.to_string_lossy().to_string()),
            ..Default::default()
        })
        .collect::<Vec<_>>();
    Chapter::insert_many(models).exec(db).await.unwrap();
    // insert the chapters
    Ok(())
}
#[cfg(test)]
mod tests {}
