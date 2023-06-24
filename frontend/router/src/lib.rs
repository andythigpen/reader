use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/rss_feeds")]
    RssFeeds,
    #[at("/rss_feeds/:id/articles")]
    RssFeedArticles { id: String },
    #[at("/rss_feeds/:id/categories")]
    RssFeedCategories { id: String },
    #[at("/categories")]
    Categories,
    #[at("/categories/:id/articles")]
    CategoryArticles { id: String },
    #[at("/articles/:id")]
    ReadabilityArticle { id: String },
}
