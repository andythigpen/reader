use anyhow::{anyhow, Result};
use dto;
use entity::{
    article, article::Entity as Article, rss_feed, rss_feed::Entity as RSSFeed, rss_feed_category,
    rss_feed_category::Entity as RSSFeedCategory,
};
use futures::future;
use nanoid::nanoid;
use reqwest::Url;
use rss::{Channel, Item};
use sea_orm::{
    sea_query::OnConflict, ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set, TransactionTrait,
};
use time::{
    format_description::well_known::Iso8601, format_description::well_known::Rfc2822,
    OffsetDateTime,
};
use urlnorm::UrlNormalizer;

use crate::article as article_service;

pub async fn create(db: &DbConn, data: dto::CreateRssFeed) -> Result<dto::RssFeed> {
    let now = OffsetDateTime::now_utc().format(&Iso8601::DEFAULT)?;
    rss_feed::ActiveModel {
        id: Set(nanoid!().to_owned()),
        name: Set(data.name.to_owned()),
        description: Set(data.description.to_owned()),
        url: Set(data.url.to_owned()),
        created_at: Set(now.to_owned()),
        updated_at: Set(now.to_owned()),
        display_description: Set(data.display_description),
        color: Set(data.color.to_owned()),
        abbreviation: Set(data.abbreviation.to_owned()),
    }
    .insert(db)
    .await
    .map_err(|e| anyhow!(e))
    .map(Into::into)
}

pub async fn find_by_id(db: &DbConn, id: &str) -> Result<Option<dto::RssFeed>, DbErr> {
    Ok(RSSFeed::find_by_id(id).one(db).await?.map(Into::into))
}

pub async fn list_by_page(
    db: &DbConn,
    page: u64,
    per_page: u64,
) -> Result<Vec<dto::RssFeed>, DbErr> {
    Ok(RSSFeed::find()
        .order_by_desc(rss_feed::Column::CreatedAt)
        .paginate(db, per_page)
        .fetch_page(page)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
}

pub async fn update_by_id(db: &DbConn, id: &str, data: dto::UpdateRssFeed) -> Result<dto::RssFeed> {
    let mut rss_feed: rss_feed::ActiveModel = RSSFeed::find_by_id(id)
        .one(db)
        .await?
        .ok_or(DbErr::Custom("Cannot find RSS feed.".to_owned()))
        .map(Into::into)?;
    rss_feed.name = Set(data.name.to_owned());
    rss_feed.description = Set(data.description.to_owned());
    rss_feed.url = Set(data.url.to_owned());
    let now = OffsetDateTime::now_utc().format(&Iso8601::DEFAULT)?;
    rss_feed.updated_at = Set(now.to_owned());
    rss_feed.display_description = Set(data.display_description);
    rss_feed.color = Set(data.color);
    rss_feed.abbreviation = Set(data.abbreviation);
    let model = rss_feed.update(db).await.map_err(|e| anyhow!(e))?;
    Ok(model.into())
}

pub async fn delete_by_id(db: &DbConn, id: &str) -> Result<()> {
    let txn = db.begin().await?;

    // remove from all categories
    RSSFeedCategory::delete_many()
        .filter(rss_feed_category::Column::RssFeedId.eq(id))
        .exec(db)
        .await?;

    // remove all articles
    article_service::delete_by_rss_feed_id(db, id).await?;

    // remove the feed
    let rss_feed: rss_feed::ActiveModel = RSSFeed::find_by_id(id)
        .one(db)
        .await?
        .ok_or(DbErr::Custom("Cannot find RSS feed.".to_owned()))
        .map(Into::into)?;
    rss_feed.delete(db).await?;

    txn.commit().await?;

    Ok(())
}

// TODO: move to article service mod
async fn save_article(
    db: &DbConn,
    rss_feed_id: &str,
    save_description: bool,
    item: &Item,
) -> Result<article::Model> {
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
    let description = if save_description {
        item.description().unwrap_or("").to_owned()
    } else {
        "".to_owned()
    };
    let created_at = OffsetDateTime::now_utc().format(&Iso8601::DEFAULT).unwrap();
    let pub_date = match item.pub_date() {
        Some(d) => OffsetDateTime::parse(d, &Rfc2822)
            .unwrap()
            .format(&Iso8601::DEFAULT)
            .unwrap(),
        None => created_at.to_owned(),
    };
    article::ActiveModel {
        id: Set(nanoid!()),
        title: Set(item.title().unwrap_or("").to_owned()),
        url: Set(item.link().unwrap_or("").to_owned()),
        normalized_url: Set(normalized_url),
        comments_url: Set(item.comments.to_owned()),
        description: Set(description),
        created_at: Set(created_at),
        pub_date: Set(pub_date),
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

    future::try_join_all(
        channel
            .items()
            .iter()
            .map(|it| async { save_article(db, id, rss_feed.display_description, it).await }),
    )
    .await
}

pub async fn fetch_all_articles(db: &DbConn) -> Result<()> {
    let rss_feeds: Vec<(String,)> = RSSFeed::find()
        .select_only()
        .columns([rss_feed::Column::Id])
        .into_tuple()
        .all(db)
        .await?;

    future::try_join_all(rss_feeds.iter().map(|(id,)| fetch_articles(db, id))).await?;

    Ok(())
}

pub async fn add_to_category(db: &DbConn, id: &str, category_id: &str) -> Result<()> {
    let result = RSSFeedCategory::insert(rss_feed_category::ActiveModel {
        rss_feed_id: Set(id.to_string()),
        category_id: Set(category_id.to_string()),
    })
    .on_conflict(
        OnConflict::columns([
            rss_feed_category::Column::RssFeedId,
            rss_feed_category::Column::CategoryId,
        ])
        .do_nothing()
        .to_owned(),
    )
    .exec(db)
    .await;

    match result {
        Ok(_) => Ok(()),
        Err(DbErr::RecordNotInserted) => Ok(()),
        Err(e) => Err(anyhow!(e)),
    }
}

pub async fn remove_from_category(db: &DbConn, id: &str, category_id: &str) -> Result<()> {
    RSSFeedCategory::delete_many()
        .filter(rss_feed_category::Column::RssFeedId.eq(id))
        .filter(rss_feed_category::Column::CategoryId.eq(category_id))
        .exec(db)
        .await?;
    Ok(())
}
