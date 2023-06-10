use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Article::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Article::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Article::Title).string().not_null())
                    .col(ColumnDef::new(Article::URL).string().not_null())
                    .col(
                        ColumnDef::new(Article::Description)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .col(ColumnDef::new(Article::CreatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Article::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Article {
    Table,
    Id,
    Title,
    URL,
    Description,
    CreatedAt,
}
