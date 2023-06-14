use gloo_net::http::Request;
use stores::article::ArticleStore;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::icons::{arrow_path::IconArrowPath, rss::IconRss};

#[function_component(Header)]
pub fn header() -> Html {
    let dispatch = Dispatch::<ArticleStore>::new();
    let onclick = dispatch.reduce_mut_future_callback(|state| {
        Box::pin(async move {
            let resp = Request::post("/api/rss_feeds/fetch").send().await.unwrap();
            if resp.status() == 200 {
                state.reload().await;
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
            <div class={classes!("flex-1")}>
                <a href="/">
                    <IconRss class={classes!("inline", "mx-2")}/>
                    {"Reader"}
                </a>
            </div>
            <div {onclick}>
                <IconArrowPath class={classes!("mx-2", "cursor-pointer")}/>
            </div>
        </div>
    }
}
