use gloo_net::http::Request;
use stores::rss_feed::RssFeedStore;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::button::Button;
use crate::list_item::ListItem;
use crate::modal::Modal;
use crate::rss_feed_form::{ModalAction, RssFeedForm};
use crate::{icons::plus::IconPlus, rss_feed::RssFeed};

#[function_component(RssFeedList)]
pub fn rss_feed_list() -> Html {
    let rss_feeds = use_selector(|s: &RssFeedStore| s.rss_feeds.clone());
    let display_modal = use_state(|| false);
    let edit_model = use_state(|| None);

    {
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    Dispatch::<RssFeedStore>::new()
                        .reduce_mut_future(|s| Box::pin(async move { s.fetch().await }))
                        .await;
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
            .map(|(id, _)| {
                html! { <RssFeed key={id} {id} /> }
            })
            .collect::<Html>()
    };

    let add_rss_feed = {
        let display_modal = display_modal.clone();
        Callback::from(move |_| display_modal.set(true))
    };

    let close_modal = {
        let display_modal = display_modal.clone();
        Callback::from(move |_| display_modal.set(false))
    };

    let close_form = {
        let display_modal = display_modal.clone();
        Callback::from(move |action| match action {
            ModalAction::Close => display_modal.set(false),
            ModalAction::Confirm(model) => {
                display_modal.set(false);
                spawn_local(async move {
                    let resp = Request::post("/api/rss_feeds")
                        .json(&model)
                        .unwrap()
                        .send()
                        .await
                        .unwrap();
                    let model = resp.json().await.unwrap();
                    Dispatch::<RssFeedStore>::new().reduce_mut(|s| s.rss_feeds.insert(0, model));
                });
            }
        })
    };

    html! {
        <>
            <Modal display={*display_modal} onclose={close_modal}>
                <RssFeedForm model={(*edit_model).to_owned()} onclose={close_form} />
            </Modal>
            <div class={classes!("flex", "flex-row", "justify-between", "items-center")}>
                <h2 class={classes!("flex-1", "text-xl")}>{"RSS Feeds"}</h2>
                <Button onclick={add_rss_feed} primary=true>
                    <IconPlus />
                    {"Add"}
                </Button>
            </div>
            { rss_feeds }
        </>
    }
}
