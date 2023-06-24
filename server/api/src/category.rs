use anyhow::Result;
use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use dto;
use std::sync::Arc;

use crate::{error::RestError, pagination::Pagination, AppState};

async fn list(
    State(state): State<Arc<AppState>>,
    pagination: Query<Pagination>,
) -> Result<Json<Vec<dto::Category>>, RestError> {
    let page =
        service::category::list_by_page(&state.conn, pagination.page, pagination.per_page).await?;
    Ok(page.into())
}

async fn create(
    State(state): State<Arc<AppState>>,
    Json(body): Json<dto::CreateCategory>,
) -> Result<Json<dto::Category>, RestError> {
    let model = service::category::create(&state.conn, body).await?;
    Ok(model.into())
}

async fn retrieve(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<dto::Category>, RestError> {
    let model = service::category::find_by_id(&state.conn, &id)
        .await?
        .ok_or(RestError::NotFound(format!("Category '{}' not found", id)))?;
    Ok(model.into())
}

async fn update(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<dto::UpdateCategory>,
) -> Result<Json<dto::Category>, RestError> {
    let model = service::category::update_by_id(&state.conn, &id, body).await?;
    Ok(model.into())
}

async fn delete(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<(), RestError> {
    service::category::delete_by_id(&state.conn, &id).await?;
    Ok(())
}

async fn list_articles(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    pagination: Query<Pagination>,
) -> Result<Json<Vec<dto::Article>>, RestError> {
    let resp = service::article::list_by_page_and_category(
        &state.conn,
        &id,
        pagination.page,
        pagination.per_page,
    )
    .await?;
    Ok(resp.into())
}

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(list).post(create))
        .route("/:id", get(retrieve).put(update).delete(delete))
        .route("/:id/articles", get(list_articles))
        .with_state(state)
}
