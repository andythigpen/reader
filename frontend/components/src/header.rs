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
    html! {
        <div class={classes!("flex", "flex-row", "dark:text-white", "w-full", "max-w-4xl", "m-1", "items-center")}>
            <IconRss class={classes!("inline", "mx-2")}/>
            <div class={classes!("flex-1")}>
                {"Reader"}
            </div>
            <div {onclick}>
                <IconArrowPath class={classes!("mx-2", "cursor-pointer")}/>
            </div>
        </div>
    }
}
