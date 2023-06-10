use anyhow::{anyhow, Result};
use entity::{article, article::Entity as Article, rss_feed, rss_feed::Entity as RSSFeed};
use futures::future;
use nanoid::nanoid;
use reqwest::Url;
use rss::{Channel, Item};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};
use time::{format_description::well_known::Iso8601, OffsetDateTime};
use urlnorm::UrlNormalizer;

pub async fn create(db: &DbConn, data: rss_feed::Model) -> Result<rss_feed::Model> {
    rss_feed::ActiveModel {
        id: Set(nanoid!().to_owned()),
        name: Set(data.name.to_owned()),
        description: Set(data.description.to_owned()),
        url: Set(data.url.to_owned()),
        created_at: Set(OffsetDateTime::now_utc().format(&Iso8601::DEFAULT)?),
    }
    .insert(db)
    .await
    .map_err(|e| anyhow!(e))
}

pub async fn find_by_id(db: &DbConn, id: &str) -> Result<Option<rss_feed::Model>, DbErr> {
    RSSFeed::find_by_id(id).one(db).await
}

pub async fn list_by_page(
    db: &DbConn,
    page: u64,
    per_page: u64,
) -> Result<Vec<rss_feed::Model>, DbErr> {
    RSSFeed::find()
        .order_by_asc(rss_feed::Column::Id)
        .paginate(db, per_page)
        .fetch_page(page)
        .await
}

pub async fn update_by_id(
    db: &DbConn,
    id: &str,
    data: rss_feed::Model,
) -> Result<rss_feed::Model, DbErr> {
    let mut rss_feed: rss_feed::ActiveModel = find_by_id(db, id)
        .await?
        .ok_or(DbErr::Custom("Cannot find RSS feed.".to_owned()))
        .map(Into::into)?;
    rss_feed.name = Set(data.name.to_owned());
    rss_feed.description = Set(data.description.to_owned());
    rss_feed.url = Set(data.url.to_owned());
    rss_feed.update(db).await
}

pub async fn delete_by_id(db: &DbConn, id: &str) -> Result<(), DbErr> {
    let rss_feed: rss_feed::ActiveModel = find_by_id(db, id)
        .await?
        .ok_or(DbErr::Custom("Cannot find RSS feed.".to_owned()))
        .map(Into::into)?;
    rss_feed.delete(db).await?;
    Ok(())
}

async fn save_article(db: &DbConn, rss_feed_id: &str, item: &Item) -> Result<article::Model> {
    let norm = UrlNormalizer::default();
    let link = item.link().unwrap_or("");
    let url = Url::parse(link)?;
    let normalized_url = norm.compute_normalization_string(&url);
    let article = Article::find()
        .filter(article::Column::NormalizedUrl.eq(normalized_url.to_owned()))
        .one(db)
        .await?;
    if let Some(article) = article {
        // already exists
        return Ok(article);
    }
    article::ActiveModel {
        id: Set(nanoid!()),
        title: Set(item.title().unwrap_or("").to_owned()),
        url: Set(item.link().unwrap_or("").to_owned()),
        normalized_url: Set(normalized_url),
        description: Set(item.description().unwrap_or("").to_owned()),
        created_at: Set(OffsetDateTime::now_utc().format(&Iso8601::DEFAULT).unwrap()),
        rss_feed_id: Set(rss_feed_id.to_owned()),
    }
    .insert(db)
    .await
    .map_err(|e| anyhow!(e))
}

pub async fn fetch_articles(db: &DbConn, id: &str) -> Result<Vec<article::Model>> {
    let rss_feed = find_by_id(db, id)
        .await?
        .ok_or(DbErr::Custom("Cannot find RSS feed.".to_owned()))?;

    let content = reqwest::get(rss_feed.url).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    // let norm = UrlNormalizer::default();

    future::try_join_all(
        channel
            .items()
            .iter()
            .map(|it| async { save_article(db, id, it).await }),
    )
    .await

    // channel
    //     .items()
    //     .iter()
    //     .map(|it| async { save_article(db, id, it).await })
    //     // .map(|it| -> Result<article::Model> {
    //     //     let link = it.link().unwrap_or("");
    //     //     let url = Url::parse(link)?;
    //     //     Ok(article::Model {
    //     //         id: nanoid!(),
    //     //         title: it.title().unwrap_or("").to_owned(),
    //     //         url: it.link().unwrap_or("").to_owned(),
    //     //         normalized_url: norm.compute_normalization_string(&url),
    //     //         description: it.description().unwrap_or("").to_owned(),
    //     //         created_at: OffsetDateTime::now_utc().format(&Iso8601::DEFAULT).unwrap(),
    //     //         rss_feed_id: id.to_owned(),
    //     //     })
    //     // })
    //     .collect()
}
