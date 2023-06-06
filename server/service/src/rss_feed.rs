use entity::{rss_feed, rss_feed::Entity as RSSFeed};
use nanoid::nanoid;
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait, PaginatorTrait, QueryOrder, Set};

pub async fn create(db: &DbConn, data: rss_feed::Model) -> Result<rss_feed::Model, DbErr> {
    rss_feed::ActiveModel {
        id: Set(nanoid!().to_owned()),
        name: Set(data.name.to_owned()),
        description: Set(data.description.to_owned()),
        url: Set(data.url.to_owned()),
    }
    .insert(db)
    .await
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
