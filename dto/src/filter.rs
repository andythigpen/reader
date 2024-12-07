use entity::filter;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateFilter {
    pub pattern: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateFilter {
    pub pattern: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Filter {
    pub id: String,
    pub pattern: String,
}

impl From<filter::Model> for Filter {
    fn from(value: filter::Model) -> Self {
        Self {
            id: value.id,
            pattern: value.pattern,
        }
    }
}
