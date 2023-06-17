use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Category::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Category::Name).string().not_null())
                    .col(
                        ColumnDef::new(Category::Description)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(RssFeedCategory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RssFeedCategory::RssFeedId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RssFeedCategory::CategoryId)
                            .string()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_rssfeedcategory_rssfeed")
                            .on_delete(ForeignKeyAction::Cascade)
                            .from(RssFeedCategory::Table, RssFeedCategory::RssFeedId)
                            .to(RssFeed::Table, RssFeed::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_rssfeedcategory_category")
                            .on_delete(ForeignKeyAction::Cascade)
                            .from(RssFeedCategory::Table, RssFeedCategory::CategoryId)
                            .to(Category::Table, Category::Id),
                    )
                    .primary_key(
                        Index::create()
                            // .name("pk_rssfeedcategory")
                            .col(RssFeedCategory::CategoryId)
                            .col(RssFeedCategory::RssFeedId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RssFeedCategory::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Category::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Category {
    Table,
    Id,
    Name,
    Description,
}

#[derive(Iden)]
enum RssFeedCategory {
    Table,
    RssFeedId,
    CategoryId,
}

#[derive(Iden)]
enum RssFeed {
    Table,
    Id,
}
