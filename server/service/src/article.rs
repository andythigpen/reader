use anyhow::Result;
use entity::{article, article::Entity as Article};
use sea_orm::{DbConn, DbErr, EntityTrait, PaginatorTrait, QueryOrder};

pub async fn find_by_id(db: &DbConn, id: &str) -> Result<Option<article::Model>, DbErr> {
    Article::find_by_id(id).one(db).await
}

pub async fn list_by_page(
    db: &DbConn,
    page: u64,
    per_page: u64,
) -> Result<Vec<article::Model>, DbErr> {
    Article::find()
        .order_by_desc(article::Column::CreatedAt)
        .paginate(db, per_page)
        .fetch_page(page)
        .await
}
