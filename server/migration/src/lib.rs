pub use sea_orm_migration::prelude::*;

mod m20220101_000001_add_rss_feed;
mod m20230607_234558_add_article;
mod m20230617_192327_add_category;
mod m20230624_175704_periodic_tasks;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_add_rss_feed::Migration),
            Box::new(m20230607_234558_add_article::Migration),
            Box::new(m20230617_192327_add_category::Migration),
            Box::new(m20230624_175704_periodic_tasks::Migration),
        ]
    }
}
