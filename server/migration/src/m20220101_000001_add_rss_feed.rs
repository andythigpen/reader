use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RSSFeed::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RSSFeed::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RSSFeed::Name).string().not_null())
                    .col(
                        ColumnDef::new(RSSFeed::Description)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .col(ColumnDef::new(RSSFeed::Url).string().not_null())
                    .col(ColumnDef::new(RSSFeed::CreatedAt).date_time().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RSSFeed::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum RSSFeed {
    Table,
    Id,
    Name,
    Description,
    Url,
    CreatedAt,
}
