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
                    .add_column_if_not_exists(ColumnDef::new(RssFeed::NextUpdate).date_time())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(RssFeed::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(RssFeed::UpdateIntervalMins)
                            .unsigned()
                            .not_null()
                            .default(360),
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
                    .drop_column(RssFeed::NextUpdate)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(RssFeed::Table)
                    .drop_column(RssFeed::UpdateIntervalMins)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum RssFeed {
    Table,
    NextUpdate,
    UpdateIntervalMins,
}
