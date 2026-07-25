use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(RssFeed::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(RssFeed::LinkDirectly)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(RssFeed::Table)
                    .drop_column(RssFeed::LinkDirectly)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum RssFeed {
    Table,
    LinkDirectly,
}
