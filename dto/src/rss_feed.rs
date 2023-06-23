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
