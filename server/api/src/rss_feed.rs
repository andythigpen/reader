use anyhow::Result;
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use std::sync::Arc;

use entity::rss_feed;

use crate::{error::RestError, AppState};

async fn list(State(state): State<Arc<AppState>>) -> Result<Json<Vec<rss_feed::Model>>, RestError> {
    Ok(vec![].into())
}

async fn create(State(state): State<Arc<AppState>>) -> Result<Json<rss_feed::Model>, RestError> {
    // TODO
    Ok(rss_feed::Model {
        id: "id".to_string(),
        name: "name".to_string(),
        description: "description".to_string(),
        url: "url".to_string(),
    }
    .into())
}

async fn retrieve(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<rss_feed::Model>, RestError> {
    // TODO
    Ok(rss_feed::Model {
        id: "id".to_string(),
        name: "name".to_string(),
        description: "description".to_string(),
        url: "url".to_string(),
    }
    .into())
}

async fn update(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<rss_feed::Model>, RestError> {
    // TODO
    Ok(rss_feed::Model {
        id: "id".to_string(),
        name: "name".to_string(),
        description: "description".to_string(),
        url: "url".to_string(),
    }
    .into())
}

async fn delete(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<(), RestError> {
    // TODO
    Ok(())
}

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(list).post(create))
        .route("/:id", get(retrieve).put(update).delete(delete))
        .with_state(state)
}
