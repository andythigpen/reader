use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use std::sync::Arc;

use crate::{error::RestError, pagination::Pagination, AppState};

async fn list(
    State(state): State<Arc<AppState>>,
    pagination: Query<Pagination>,
) -> Result<Json<Vec<dto::Filter>>, RestError> {
    let page =
        service::filter::list_by_page(&state.conn, pagination.page, pagination.per_page).await?;
    Ok(page.into())
}

async fn create(
    State(state): State<Arc<AppState>>,
    Json(body): Json<dto::CreateFilter>,
) -> Result<Json<dto::Filter>, RestError> {
    let model = service::filter::create(&state.conn, body).await?;
    Ok(model.into())
}

async fn retrieve(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<dto::Filter>, RestError> {
    let model = service::filter::find_by_id(&state.conn, &id)
        .await?
        .ok_or(RestError::NotFound(format!("Filter '{}' not found", id)))?;
    Ok(model.into())
}

async fn update(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<dto::UpdateFilter>,
) -> Result<Json<dto::Filter>, RestError> {
    let model = service::filter::update_by_id(&state.conn, &id, body).await?;
    Ok(model.into())
}

async fn delete(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<(), RestError> {
    service::filter::delete_by_id(&state.conn, &id).await?;
    Ok(())
}

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(list).post(create))
        .route("/:id", get(retrieve).put(update).delete(delete))
        .with_state(state)
}
