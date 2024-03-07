use sea_orm_migration::prelude::*;

use crate::m20230917_000003_create_music_table::Music;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Chapter::Table)
                    .col(
                        ColumnDef::new(Chapter::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Chapter::MusicId).integer().not_null())
                    .col(ColumnDef::new(Chapter::ChapterNum).integer().not_null())
                    .col(ColumnDef::new(Chapter::ChapterName).string().not_null())
                    .col(ColumnDef::new(Chapter::ChapterUrl).string().not_null())
                    .col(ColumnDef::new(Chapter::ChapterLength).float())
                    .foreign_key(
                        ForeignKey::create()
                            .name("music_chapter_music_id_fkey")
                            .from(Chapter::Table, Chapter::MusicId)
                            .to(Music::Table, Music::Id),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_music_chapter_music_id")
                    .table(Chapter::Table)
                    .col(Chapter::MusicId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_music_chapter_chapter_num")
                    .table(Chapter::Table)
                    .col(Chapter::ChapterNum)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Chapter::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Chapter {
    Table,
    Id,
    MusicId,
    ChapterNum,
    ChapterName,
    ChapterUrl,
    ChapterLength,
}
