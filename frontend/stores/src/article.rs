use entity::article::Model as Article;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yewdux::{log::info, prelude::*};

#[derive(Debug, Clone, PartialEq, Eq, Store)]
pub struct ArticleStore {
    pub articles: Vec<Article>,
    // true when a fetch is currently in progress
    pub fetching: bool,
    // true when the last attempt to fetch articles returns less than the per_page amount...meaning
    // no more articles are available
    pub at_end: bool,
    pub per_page: u64,
    page: u64,
}

impl Default for ArticleStore {
    fn default() -> Self {
        Self {
            articles: vec![],
            fetching: false,
            at_end: false,
            per_page: 20,
            page: 1,
        }
    }
}

impl ArticleStore {
    pub fn fetch(&mut self) {
        if self.fetching || self.at_end {
            info!("fetching={} at_end={}", self.fetching, self.at_end);
            return;
        }
        let page = self.page;
        let per_page = self.per_page;
        spawn_local(async move {
            let dispatch = Dispatch::<ArticleStore>::new();
            let resp = Request::get("/api/articles")
                .query([
                    ("page", page.to_string()),
                    ("per_page", per_page.to_string()),
                ])
                .send()
                .await
                .unwrap();
            let result: Result<Vec<Article>, String> = {
                if !resp.ok() {
                    Err(format!(
                        "Error fetching data {} ({})",
                        resp.status(),
                        resp.status_text()
                    ))
                } else {
                    resp.json().await.map_err(|err| err.to_string())
                }
            };
            if let Ok(mut articles) = result {
                dispatch.reduce_mut(|s| {
                    s.page += 1;
                    let len = articles.len();
                    s.at_end = len as u64 != per_page;
                    s.articles.append(&mut articles);
                    s.fetching = false;
                });
            }
        });
    }
}
