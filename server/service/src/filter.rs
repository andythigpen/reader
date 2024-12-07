use anyhow::{anyhow, Result};
use entity::{filter, filter::Entity as Filter};
use nanoid::nanoid;
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait, PaginatorTrait, QueryOrder, Set};

pub async fn list_all(db: &DbConn) -> Result<Vec<dto::Filter>, DbErr> {
    Ok(Filter::find()
        .order_by_desc(filter::Column::Id)
        .all(db)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
}

pub async fn list_by_page(
    db: &DbConn,
    page: u64,
    per_page: u64,
) -> Result<Vec<dto::Filter>, DbErr> {
    Ok(Filter::find()
        .order_by_desc(filter::Column::Id)
        .paginate(db, per_page)
        .fetch_page(page)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
}

pub async fn create(db: &DbConn, data: dto::CreateFilter) -> Result<dto::Filter> {
    filter::ActiveModel {
        id: Set(nanoid!().to_owned()),
        keyword: Set(data.keyword.to_owned()),
    }
    .insert(db)
    .await
    .map_err(|e| anyhow!(e))
    .map(Into::into)
}

pub async fn find_by_id(db: &DbConn, id: &str) -> Result<Option<dto::Filter>, DbErr> {
    Ok(Filter::find_by_id(id).one(db).await?.map(Into::into))
}

pub async fn update_by_id(db: &DbConn, id: &str, data: dto::UpdateFilter) -> Result<dto::Filter> {
    let mut filter: filter::ActiveModel = Filter::find_by_id(id)
        .one(db)
        .await?
        .ok_or(DbErr::Custom("Cannot find filter.".to_owned()))
        .map(Into::into)?;
    filter.keyword = Set(data.keyword.to_owned());
    filter
        .update(db)
        .await
        .map_err(|e| anyhow!(e))
        .map(Into::into)
}

pub async fn delete_by_id(db: &DbConn, id: &str) -> Result<()> {
    let filter: filter::ActiveModel = Filter::find_by_id(id)
        .one(db)
        .await?
        .ok_or(DbErr::Custom("Cannot find filter.".to_owned()))
        .map(Into::into)?;
    filter.delete(db).await?;
    Ok(())
}
