use anyhow::Result;
use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;

use entity::{article, rss_feed};

use crate::{error::RestError, pagination::Pagination, AppState};

async fn list(
    State(state): State<Arc<AppState>>,
    pagination: Query<Pagination>,
) -> Result<Json<Vec<rss_feed::Model>>, RestError> {
    let page =
        service::rss_feed::list_by_page(&state.conn, pagination.page, pagination.per_page).await?;
    Ok(page.into())
}

async fn create(
    State(state): State<Arc<AppState>>,
    Json(body): Json<rss_feed::Model>,
) -> Result<Json<rss_feed::Model>, RestError> {
    let model = service::rss_feed::create(&state.conn, body).await?;
    Ok(model.into())
}

async fn retrieve(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<rss_feed::Model>, RestError> {
    let model = service::rss_feed::find_by_id(&state.conn, &id)
        .await?
        .ok_or(RestError::NotFound(format!("RSS feed '{}' not found", id)))?;
    Ok(model.into())
}

async fn update(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(body): Json<rss_feed::Model>,
) -> Result<Json<rss_feed::Model>, RestError> {
    let model = service::rss_feed::update_by_id(&state.conn, &id, body).await?;
    Ok(model.into())
}

async fn delete(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<(), RestError> {
    service::rss_feed::delete_by_id(&state.conn, &id).await?;
    Ok(())
}

async fn fetch_articles(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<article::Model>>, RestError> {
    let resp = service::rss_feed::fetch_articles(&state.conn, &id).await?;
    Ok(resp.into())
}

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(list).post(create))
        .route("/:id", get(retrieve).put(update).delete(delete))
        .route("/:id/fetch", post(fetch_articles))
        .with_state(state)
}
