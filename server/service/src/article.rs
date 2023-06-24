use anyhow::{anyhow, Result};
use dto;
use entity::{
    article::Column, article::Entity as Article, rss_feed::Entity as RssFeed, rss_feed_category,
};
use readability::extractor;
use sea_orm::{
    sea_query::{Expr, IntoCondition},
    ColumnTrait, DbConn, DbErr, EntityTrait, JoinType, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, RelationTrait,
};
use time::{format_description::well_known::Iso8601, OffsetDateTime};
use tokio::task;

pub async fn find_by_id(db: &DbConn, id: &str) -> Result<Option<dto::Article>, DbErr> {
    Ok(Article::find_by_id(id).one(db).await?.map(Into::into))
}

pub async fn list_by_page(
    db: &DbConn,
    page: u64,
    per_page: u64,
) -> Result<Vec<dto::Article>, DbErr> {
    Ok(Article::find()
        .order_by_desc(Column::PubDate)
        .paginate(db, per_page)
        .fetch_page(page)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
}

pub async fn list_by_page_and_rss_feed(
    db: &DbConn,
    rss_feed_id: &str,
    page: u64,
    per_page: u64,
) -> Result<Vec<dto::Article>, DbErr> {
    Ok(Article::find()
        .filter(Column::RssFeedId.eq(rss_feed_id))
        .order_by_desc(Column::PubDate)
        .paginate(db, per_page)
        .fetch_page(page)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
}

pub async fn list_by_page_and_category(
    db: &DbConn,
    category_id: &str,
    page: u64,
    per_page: u64,
) -> Result<Vec<dto::Article>, DbErr> {
    let category_id = category_id.to_owned();
    Ok(Article::find()
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
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
}

pub async fn delete_by_rss_feed_id(db: &DbConn, rss_feed_id: &str) -> Result<()> {
    Article::delete_many()
        .filter(Column::RssFeedId.eq(rss_feed_id))
        .exec(db)
        .await?;
    Ok(())
}

pub async fn delete_old_articles(db: &DbConn) -> Result<()> {
    let oldest = OffsetDateTime::now_utc()
        .saturating_sub(time::Duration::days(7))
        .format(&Iso8601::DEFAULT)?;
    Article::delete_many()
        .filter(Column::CreatedAt.lt(oldest))
        .exec(db)
        .await?;
    Ok(())
}

pub async fn get_readability_article(db: &DbConn, id: &str) -> Result<dto::ReadabilityArticle> {
    let article = find_by_id(db, id)
        .await?
        .ok_or(anyhow!("Article not found"))?;
    let url = article.url.clone();
    let scrape = task::spawn_blocking(move || extractor::scrape(&article.url))
        .await?
        .map_err(|e| anyhow!(e))?;
    Ok(dto::ReadabilityArticle {
        pub_date: article.pub_date,
        rss_feed_id: article.rss_feed_id,
        url,
        title: scrape.title,
        content: scrape.content,
        text: scrape.text,
    })
}
