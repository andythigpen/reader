use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Article {
    pub id: String,
    pub title: String,
    pub url: String,
    pub normalized_url: String,
    pub description: String,
    pub created_at: String,
    pub pub_date: String,
    pub comments_url: Option<String>,
    pub rss_feed_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadabilityArticle {
    pub pub_date: String,
    pub rss_feed_id: String,
    pub url: String,
    pub title: String,
    pub content: String,
    pub text: String,
}
