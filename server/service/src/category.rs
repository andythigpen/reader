use anyhow::{anyhow, Result};
use entity::{category, category::Entity as Category};
use nanoid::nanoid;
use sea_orm::{
    ActiveModelTrait, DbConn, DbErr, EntityTrait, ModelTrait, PaginatorTrait, QueryOrder, Set,
};
use serde::{Deserialize, Serialize};

use crate::rss_feed as rss_feed_service;

#[derive(Serialize, Deserialize)]
pub struct CreateModel {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateModel {
    pub name: String,
    pub description: String,
}

pub async fn create(db: &DbConn, data: CreateModel) -> Result<category::Model> {
    category::ActiveModel {
        id: Set(nanoid!().to_owned()),
        name: Set(data.name.to_owned()),
        description: Set(data.description.to_owned()),
    }
    .insert(db)
    .await
    .map_err(|e| anyhow!(e))
}

pub async fn find_by_id(db: &DbConn, id: &str) -> Result<Option<category::Model>, DbErr> {
    Category::find_by_id(id).one(db).await
}

pub async fn list_by_page(
    db: &DbConn,
    page: u64,
    per_page: u64,
) -> Result<Vec<category::Model>, DbErr> {
    Category::find()
        .order_by_desc(category::Column::Id)
        .paginate(db, per_page)
        .fetch_page(page)
        .await
}

pub async fn update_by_id(db: &DbConn, id: &str, data: UpdateModel) -> Result<category::Model> {
    let mut category: category::ActiveModel = find_by_id(db, id)
        .await?
        .ok_or(DbErr::Custom("Cannot find category.".to_owned()))
        .map(Into::into)?;
    category.name = Set(data.name.to_owned());
    category.description = Set(data.description.to_owned());
    category.update(db).await.map_err(|e| anyhow!(e))
}

pub async fn delete_by_id(db: &DbConn, id: &str) -> Result<()> {
    let category: category::ActiveModel = find_by_id(db, id)
        .await?
        .ok_or(DbErr::Custom("Cannot find category.".to_owned()))
        .map(Into::into)?;
    category.delete(db).await?;
    Ok(())
}

pub async fn list_by_rss_feed(db: &DbConn, rss_feed_id: &str) -> Result<Vec<category::Model>> {
    let rss_feed = rss_feed_service::find_by_id(db, rss_feed_id)
        .await?
        .ok_or(anyhow!("Cannot find RSS feed."))?;
    rss_feed
        .find_related(Category)
        .all(db)
        .await
        .map_err(|e| anyhow!(e))
}