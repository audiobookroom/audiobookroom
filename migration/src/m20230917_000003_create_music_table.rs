// m20220101_000002_create_chef_table.rs

use sea_orm_migration::prelude::*;

use crate::m20230917_000002_create_author::Author;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230917_000003_create_music_table" // Make sure this matches with the file name
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Chef table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Music::Table)
                    .col(
                        ColumnDef::new(Music::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Music::AuthorId).integer().not_null())
                    .col(ColumnDef::new(Music::Name).string().not_null())
                    .col(ColumnDef::new(Music::Chapters).integer().not_null())
                    .col(ColumnDef::new(Music::TotalTime).float())
                    .col(ColumnDef::new(Music::FileFolder).string().not_null())
                    .col(ColumnDef::new(Music::MusicType).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_music_author_id")
                            .from(Music::Table, Music::AuthorId)
                            .to(Author::Table, Author::Id),
                    )
                    .to_owned(),
            )
            .await?;
        // create index for Name
        manager
            .create_index(
                Index::create()
                    .name("idx_music_name")
                    .table(Music::Table)
                    .col(Music::Name)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    // Define how to rollback this migration: Drop the Music table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Music::Table).to_owned())
            .await
    }
}

// For ease of access
#[derive(Iden)]
pub enum Music {
    Table,
    Id,
    AuthorId,
    Name,
    Chapters,
    TotalTime,
    FileFolder,
    MusicType,
}
