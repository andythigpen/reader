use entity::rss_feed;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateRssFeed {
    pub name: String,
    pub description: String,
    pub url: String,
    pub display_description: bool,
    pub color: String,
    pub abbreviation: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateRssFeed {
    pub name: String,
    pub description: String,
    pub url: String,
    pub display_description: bool,
    pub color: String,
    pub abbreviation: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RssFeed {
    pub id: String,
    pub name: String,
    pub description: String,
    pub url: String,
    pub created_at: String,
    pub updated_at: String,
    pub display_description: bool,
    pub abbreviation: String,
    pub color: String,
}

impl From<rss_feed::Model> for RssFeed {
    fn from(value: rss_feed::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            url: value.url,
            created_at: value.created_at,
            updated_at: value.updated_at,
            display_description: value.display_description,
            abbreviation: value.abbreviation,
            color: value.color,
        }
    }
}
