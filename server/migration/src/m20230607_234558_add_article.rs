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
                    .col(ColumnDef::new(Article::Url).string().not_null())
                    .col(ColumnDef::new(Article::NormalizedUrl).string().not_null())
                    .col(
                        ColumnDef::new(Article::Description)
                            .string()
                            .not_null()
                            .default(""),
                    )
                    .col(ColumnDef::new(Article::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Article::PubDate).date_time().not_null())
                    .col(ColumnDef::new(Article::CommentsUrl).string())
                    .col(ColumnDef::new(Article::RssFeedId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_article_rss_feed")
                            .on_delete(ForeignKeyAction::Cascade)
                            .from(Article::Table, Article::RssFeedId)
                            .to(RSSFeed::Table, RSSFeed::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_article_normalized_url")
                    .table(Article::Table)
                    .col(Article::NormalizedUrl)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_article_normalized_url")
                    .table(Article::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Article::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum RSSFeed {
    Table,
    Id,
}

#[derive(Iden)]
enum Article {
    Table,
    Id,
    Title,
    Url,
    NormalizedUrl,
    CommentsUrl,
    Description,
    CreatedAt,
    PubDate,
    RssFeedId,
}
