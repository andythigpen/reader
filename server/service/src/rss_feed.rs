use entity::rss_feed;
use sea_orm::{DbConn, DbErr};

pub async fn create(db: &DbConn, data: rss_feed::Model) -> Result<rss_feed::Model, DbErr> {
    todo!();
}
