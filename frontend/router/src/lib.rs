use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/rss_feeds")]
    RssFeeds,
    #[at("/categories")]
    Categories,
}
