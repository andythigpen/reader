use anyhow::{anyhow, Result};
use dto;
use entity::{category, category::Entity as Category, rss_feed::Entity as RSSFeed};
use nanoid::nanoid;
use sea_orm::{
    ActiveModelTrait, DbConn, DbErr, EntityTrait, ModelTrait, PaginatorTrait, QueryOrder, Set,
};

pub async fn create(db: &DbConn, data: dto::CreateCategory) -> Result<dto::Category> {
    category::ActiveModel {
        id: Set(nanoid!().to_owned()),
        name: Set(data.name.to_owned()),
        description: Set(data.description.to_owned()),
    }
    .insert(db)
    .await
    .map_err(|e| anyhow!(e))
    .map(Into::into)
}

pub async fn find_by_id(db: &DbConn, id: &str) -> Result<Option<dto::Category>, DbErr> {
    Ok(Category::find_by_id(id).one(db).await?.map(Into::into))
}

pub async fn list_by_page(
    db: &DbConn,
    page: u64,
    per_page: u64,
) -> Result<Vec<dto::Category>, DbErr> {
    Ok(Category::find()
        .order_by_desc(category::Column::Id)
        .paginate(db, per_page)
        .fetch_page(page)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
}

pub async fn update_by_id(
    db: &DbConn,
    id: &str,
    data: dto::UpdateCategory,
) -> Result<dto::Category> {
    let mut category: category::ActiveModel = Category::find_by_id(id)
        .one(db)
        .await?
        .ok_or(DbErr::Custom("Cannot find category.".to_owned()))
        .map(Into::into)?;
    category.name = Set(data.name.to_owned());
    category.description = Set(data.description.to_owned());
    category
        .update(db)
        .await
        .map_err(|e| anyhow!(e))
        .map(Into::into)
}

pub async fn delete_by_id(db: &DbConn, id: &str) -> Result<()> {
    let category: category::ActiveModel = Category::find_by_id(id)
        .one(db)
        .await?
        .ok_or(DbErr::Custom("Cannot find category.".to_owned()))
        .map(Into::into)?;
    category.delete(db).await?;
    Ok(())
}

pub async fn list_by_rss_feed(db: &DbConn, rss_feed_id: &str) -> Result<Vec<dto::Category>> {
    let rss_feed = RSSFeed::find_by_id(rss_feed_id)
        .one(db)
        .await?
        .ok_or(anyhow!("Cannot find RSS feed."))?;
    Ok(rss_feed
        .find_related(Category)
        .all(db)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
}
