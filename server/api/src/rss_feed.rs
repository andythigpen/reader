use anyhow::Result;
use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use dto;
use std::sync::Arc;

use entity::{article, category, rss_feed};

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
    Json(body): Json<dto::CreateRssFeed>,
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
    Json(body): Json<dto::UpdateRssFeed>,
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

async fn fetch_all(State(state): State<Arc<AppState>>) -> Result<(), RestError> {
    service::rss_feed::fetch_all_articles(&state.conn)
        .await
        .map_err(|err| RestError::Internal(err))
}

async fn list_categories(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<category::Model>>, RestError> {
    let resp = service::category::list_by_rss_feed(&state.conn, &id).await?;
    Ok(resp.into())
}

async fn add_category(
    Path((id, category_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
) -> Result<(), RestError> {
    service::rss_feed::add_to_category(&state.conn, &id, &category_id).await?;
    Ok(())
}

async fn remove_category(
    Path((id, category_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
) -> Result<(), RestError> {
    service::rss_feed::remove_from_category(&state.conn, &id, &category_id).await?;
    Ok(())
}

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(list).post(create))
        .route("/fetch", post(fetch_all))
        .route("/:id", get(retrieve).put(update).delete(delete))
        .route("/:id/fetch", post(fetch_articles))
        .route("/:id/categories", get(list_categories))
        .route(
            "/:id/categories/:category_id",
            post(add_category).delete(remove_category),
        )
        .with_state(state)
}
