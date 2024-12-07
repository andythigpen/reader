use dto::Filter;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Store, Serialize, Deserialize)]
pub struct FilterStore {
    pub filters: Vec<Filter>,
    // true when a fetch is currently in progress
    pub fetching: bool,
    // true when the last attempt to fetch items returns less than the per_page amount...meaning
    // no more items are available
    pub at_end: bool,
    pub per_page: u64,
    page: u64,
}

impl Default for FilterStore {
    fn default() -> Self {
        Self {
            filters: vec![],
            fetching: false,
            at_end: false,
            per_page: 1000,
            page: 0,
        }
    }
}

impl FilterStore {
    pub fn get_by_id(&self, id: &str) -> Option<Filter> {
        self.filters.iter().find(|r| r.id == id).cloned()
    }

    pub async fn reload(&mut self) {
        self.filters.clear();
        self.fetching = false;
        self.at_end = false;
        self.page = 0;
        self.fetch().await;
    }

    pub async fn fetch(&mut self) {
        if self.fetching || self.at_end {
            return;
        }

        Dispatch::<FilterStore>::new().reduce_mut(|s| s.fetching = true);

        let page = self.page;
        let per_page = self.per_page;

        let resp = Request::get("/api/filters")
            .query([
                ("page", page.to_string()),
                ("per_page", per_page.to_string()),
            ])
            .send()
            .await
            .unwrap();
        let result: Result<Vec<Filter>, String> = {
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
        if let Ok(mut filters) = result {
            self.page += 1;
            let len = filters.len();
            self.at_end = len as u64 != per_page;
            self.filters.append(&mut filters);
        }
        self.fetching = false;
    }
}
