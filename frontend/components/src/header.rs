use gloo_net::http::Request;
use router::Route;
use stores::article::ArticleStore;
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::icons::{bars_3::IconBars3, rss::IconRss};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Header)]
pub fn header(Props { children }: &Props) -> Html {
    let dispatch = Dispatch::<ArticleStore>::new();
    let onclick = dispatch.reduce_mut_future_callback(|state| {
        Box::pin(async move {
            let resp = Request::post("/api/rss_feeds/fetch").send().await.unwrap();
            if resp.status() == 200 {
                state.reload().await;
                window().unwrap().scroll_with_x_and_y(0.0, 0.0);
            }
        })
    });
    let classes = classes!(
        "flex",
        "flex-row",
        "dark:text-white",
        "dark:bg-slate-950",
        "w-full",
        "max-w-5xl",
        "p-1",
        "items-center",
        "sticky",
        "top-0",
        "relative",
        // inverted border below
        "before:content-['']",
        "before:bg-transparent",
        "before:rounded-t-lg",
        "before:absolute",
        "before:max-w-5xl",
        "before:w-full",
        "before:inset-x-0",
        "before:-bottom-6",
        "before:h-6",
        "before:shadow-[0px_-5px_0px_0px]",
        "before:shadow-slate-950",
    );
    html! {
        <div class={classes}>
            <div class={classes!("flex-0")}>
                <Link<Route> to={Route::Home}>
                    <IconRss class={classes!("inline", "mx-2")}/>
                    {"Reader"}
                </Link<Route>>
            </div>
            <div class={classes!("flex", "flex-col", "flex-1", "relative", "items-center")}>
                {for children.iter()}
            </div>
            <div {onclick}>
                <IconBars3 class={classes!("mx-2", "cursor-pointer")}/>
            </div>
        </div>
    }
}
