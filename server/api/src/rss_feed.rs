use anyhow::Result;
use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;

use entity::{article, rss_feed};

use crate::{error::RestError, AppState};

// TODO: move to common
#[derive(Deserialize)]
struct Pagination {
    #[serde(default = "default_page")]
    page: u64,
    #[serde(default = "default_per_page")]
    per_page: u64,
}

fn default_page() -> u64 {
    0
}

fn default_per_page() -> u64 {
    10
}

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
    let model = service::rss_feed::find_by_id(&state.conn, &id).await?;
    match model {
        Some(m) => Ok(m.into()),
        None => Err(RestError::NotFound(format!("RSS feed '{}' not found", id))),
    }
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
        .route("/:id/fetch", get(fetch_articles))
        .with_state(state)
}
