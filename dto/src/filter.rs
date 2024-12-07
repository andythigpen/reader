use entity::filter;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateFilter {
    pub keyword: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateFilter {
    pub keyword: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Filter {
    pub id: String,
    pub keyword: String,
}

impl From<filter::Model> for Filter {
    fn from(value: filter::Model) -> Self {
        Self {
            id: value.id,
            keyword: value.keyword,
        }
    }
}
