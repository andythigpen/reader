use anyhow::Result;
use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use std::sync::Arc;

use entity::category;

use crate::{error::RestError, pagination::Pagination, AppState};

async fn list(
    State(state): State<Arc<AppState>>,
    pagination: Query<Pagination>,
) -> Result<Json<Vec<category::Model>>, RestError> {
    let page =
        service::category::list_by_page(&state.conn, pagination.page, pagination.per_page).await?;
    Ok(page.into())
}

async fn create(
    State(state): State<Arc<AppState>>,
    Json(body): Json<service::category::CreateModel>,
) -> Result<Json<category::Model>, RestError> {
    let model = service::category::create(&state.conn, body).await?;
    Ok(model.into())
}

async fn retrieve(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<category::Model>, RestError> {
    let model = service::category::find_by_id(&state.conn, &id)
        .await?
        .ok_or(RestError::NotFound(format!("Category '{}' not found", id)))?;
    Ok(model.into())
}

async fn update(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<service::category::UpdateModel>,
) -> Result<Json<category::Model>, RestError> {
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

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(list).post(create))
        .route("/:id", get(retrieve).put(update).delete(delete))
        .with_state(state)
}
