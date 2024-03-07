// migration/src/lib.rs

pub use sea_orm_migration::prelude::*;

// Add each migration file as a module
mod m20230917_000001_create_account_table;
mod m20230917_000002_create_author;
mod m20230917_000003_create_music_table;
mod m20240208_000001_create_progress_table;
mod m20240207_235046_create_music_chapter;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230917_000001_create_account_table::Migration),
            Box::new(m20230917_000002_create_author::Migration),
            Box::new(m20230917_000003_create_music_table::Migration),
            Box::new(m20240207_235046_create_music_chapter::Migration),
            Box::new(m20240208_000001_create_progress_table::Migration),
        ]
    }
}
