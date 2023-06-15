use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use entity::rss_feed::Model;

use crate::list_item::ListItem;
use crate::{icons::plus::IconPlus, rss_feed::RssFeed};

#[function_component(RssFeedList)]
pub fn rss_feed_list() -> Html {
    let rss_feeds = use_state(|| vec![]);

    {
        let rss_feeds = rss_feeds.clone();
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    let fetched: Vec<Model> = Request::get("/api/rss_feeds")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    rss_feeds.set(fetched);
                });
                || ()
            },
            (),
        );
    }

    let rss_feeds = if rss_feeds.is_empty() {
        html! {
            <ListItem>{"No RSS feeds. Click the Add button above to add one."}</ListItem>
        }
    } else {
        rss_feeds
            .iter()
            .enumerate()
            .map(|(id, model)| {
                html! { <RssFeed key={id} model={model.clone()}/> }
            })
            .collect::<Html>()
    };

    html! {
        <>
            <div class={classes!("flex", "flex-row", "justify-between", "items-center")}>
                <h2 class={classes!("flex-1", "text-xl")}>{"RSS Feeds"}</h2>
                <button class={classes!("flex", "flex-row", "bg-sky-500", "hover:bg-sky-400", "transition-colors", "py-2", "px-4", "rounded-xl", "font-semibold", "gap-1")}>
                    <IconPlus />
                    {"Add"}
                </button>
            </div>
            { rss_feeds }
        </>
    }
}
