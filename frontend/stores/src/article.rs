use entity::article::Model as Article;
use yewdux::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Store)]
pub struct ArticleStore {
    pub articles: Vec<Article>,
}
