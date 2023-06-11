use entity::article::Model as Article;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yewdux::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Store)]
pub struct ArticleStore {
    pub articles: Vec<Article>,
    pub per_page: u64,
    page: u64,
}

impl Default for ArticleStore {
    fn default() -> Self {
        Self {
            articles: vec![],
            per_page: 20,
            page: 1,
        }
    }
}

impl ArticleStore {
    pub fn fetch(&mut self) {
        let page = self.page.to_string();
        let per_page = self.per_page.to_string();
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
                    s.articles.append(&mut articles)
                });
            }
        });
    }
}
