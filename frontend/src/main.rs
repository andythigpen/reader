use yew::prelude::*;
use yew_router::prelude::*;

use pages::article::Article;
use pages::categories::Categories;
use pages::home::Home;
use pages::rss_feed_categories::RssFeedCategories;
use pages::rss_feeds::RssFeeds;
use router::Route;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::RssFeeds => html! { <RssFeeds /> },
        Route::RssFeedCategories { id } => html! { <RssFeedCategories {id} /> },
        Route::Categories => html! { <Categories /> },
        Route::CategoryArticles { id } => html! { <Home category_id={id} /> },
        Route::ReadabilityArticle { id } => html! { <Article {id} /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
