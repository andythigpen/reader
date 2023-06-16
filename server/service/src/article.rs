use anyhow::Result;
use entity::{article::Column, article::Entity as Article, article::Model};
use sea_orm::{ColumnTrait, DbConn, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};

pub async fn find_by_id(db: &DbConn, id: &str) -> Result<Option<Model>, DbErr> {
    Article::find_by_id(id).one(db).await
}

pub async fn list_by_page(db: &DbConn, page: u64, per_page: u64) -> Result<Vec<Model>, DbErr> {
    Article::find()
        .order_by_desc(Column::CreatedAt)
        .paginate(db, per_page)
        .fetch_page(page)
        .await
}

pub async fn delete_by_rss_feed_id(db: &DbConn, rss_feed_id: &str) -> Result<()> {
    Article::delete_many()
        .filter(Column::RssFeedId.eq(rss_feed_id))
        .exec(db)
        .await?;
    Ok(())
}
