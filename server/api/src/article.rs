use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use entity::article;

use crate::{error::RestError, pagination::Pagination, AppState};

async fn list(
    State(state): State<Arc<AppState>>,
    pagination: Query<Pagination>,
) -> Result<Json<Vec<article::Model>>, RestError> {
    let page =
        service::article::list_by_page(&state.conn, pagination.page, pagination.per_page).await?;
    Ok(page.into())
}

async fn retrieve(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<article::Model>, RestError> {
    let model = service::article::find_by_id(&state.conn, &id)
        .await?
        .ok_or(RestError::NotFound(format!("Article '{}' not found", id)))?;
    Ok(model.into())
}

async fn retrieve_readability(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<dto::ReadabilityArticle>, RestError> {
    let model = service::article::get_readability_article(&state.conn, &id).await?;
    Ok(model.into())
}

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(list))
        .route("/:id", get(retrieve))
        .route("/:id/readability", get(retrieve_readability))
        .with_state(state)
}
