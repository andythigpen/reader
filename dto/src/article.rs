use entity::article;
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

impl From<article::Model> for Article {
    fn from(value: article::Model) -> Self {
        Self {
            id: value.id,
            title: value.title,
            url: value.url,
            normalized_url: value.normalized_url,
            description: value.description,
            created_at: value.created_at,
            pub_date: value.pub_date,
            comments_url: value.comments_url,
            rss_feed_id: value.rss_feed_id,
        }
    }
}
