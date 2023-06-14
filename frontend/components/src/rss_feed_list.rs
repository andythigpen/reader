use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use entity::rss_feed::Model;

use crate::rss_feed::RssFeed;

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

    let rss_feeds = rss_feeds
        .iter()
        .enumerate()
        .map(|(id, _)| html! { <RssFeed key={id} /> })
        .collect::<Html>();

    html! {
        <>
            { rss_feeds }
        </>
    }
}
