use anyhow::Result;
use entity::{
    article::Column, article::Entity as Article, article::Model, rss_feed::Entity as RssFeed,
    rss_feed_category,
};
use sea_orm::{
    sea_query::{Expr, IntoCondition},
    ColumnTrait, DbConn, DbErr, EntityTrait, JoinType, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, RelationTrait,
};

pub async fn find_by_id(db: &DbConn, id: &str) -> Result<Option<Model>, DbErr> {
    Article::find_by_id(id).one(db).await
}

pub async fn list_by_page(db: &DbConn, page: u64, per_page: u64) -> Result<Vec<Model>, DbErr> {
    Article::find()
        .order_by_desc(Column::PubDate)
        .paginate(db, per_page)
        .fetch_page(page)
        .await
}

pub async fn list_by_page_and_category(
    db: &DbConn,
    category_id: &str,
    page: u64,
    per_page: u64,
) -> Result<Vec<Model>, DbErr> {
    let category_id = category_id.to_owned();
    Article::find()
        .left_join(RssFeed)
        .join(
            JoinType::InnerJoin,
            rss_feed_category::Relation::RssFeed
                .def()
                .rev()
                .on_condition(move |_left, _right| {
                    Expr::col(rss_feed_category::Column::CategoryId)
                        .eq(category_id.clone())
                        .into_condition()
                }),
        )
        .order_by_desc(Column::PubDate)
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
