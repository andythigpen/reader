use dto::Article;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Store, Serialize, Deserialize)]
#[store(storage = "session")]
pub struct ArticleStore {
    pub articles: Vec<Article>,
    category_id: Option<String>,
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
            category_id: None,
            fetching: false,
            at_end: false,
            per_page: 20,
            page: 0,
        }
    }
}

impl ArticleStore {
    pub fn by_article_id(&self, id: &str) -> Option<Article> {
        self.articles.iter().find(|a| a.id == id).cloned()
    }

    pub fn category_id(&mut self, category_id: Option<String>) {
        if self.category_id != category_id {
            self.articles.clear();
            self.fetching = false;
            self.at_end = false;
            self.page = 0;
        }
        self.category_id = category_id;
    }

    pub async fn reload(&mut self) {
        self.articles.clear();
        self.fetching = false;
        self.at_end = false;
        self.page = 0;
        self.fetch().await;
    }

    pub async fn fetch(&mut self) {
        if self.fetching || self.at_end {
            return;
        }

        Dispatch::<ArticleStore>::new().reduce_mut(|s| s.fetching = true);

        let page = self.page;
        let per_page = self.per_page;

        let endpoint = if let Some(category_id) = &self.category_id {
            format!("/api/categories/{}/articles", category_id)
        } else {
            "/api/articles".to_string()
        };

        let resp = Request::get(&endpoint)
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
            self.page += 1;
            let len = articles.len();
            self.at_end = len as u64 != per_page;
            self.articles.append(&mut articles);
        }
        self.fetching = false;
    }
}
