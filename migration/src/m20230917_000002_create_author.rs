// m20220101_000002_create_chef_table.rs

use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230917_000002_create_author" // Make sure this matches with the file name
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Chef table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Author::Table)
                    .col(
                        ColumnDef::new(Author::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Author::Avatar).string().not_null())
                    .col(ColumnDef::new(Author::Name).string().not_null())
                    .col(ColumnDef::new(Author::Description).string().not_null())
                    .to_owned(),
            )
            .await?;

        // create index for Name
        manager
            .create_index(
                Index::create()
                    .name("idx_author_name")
                    .table(Author::Table)
                    .col(Author::Name)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    // Define how to rollback this migration: Drop the Music table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Author::Table).to_owned())
            .await
    }
}

// For ease of access
#[derive(Iden)]
pub enum Author {
    Table,
    Id,
    Name,
    Avatar,
    Description,
}
