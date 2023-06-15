use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use entity::rss_feed::Model;

use crate::list_item::ListItem;
use crate::modal::Modal;
use crate::{icons::plus::IconPlus, rss_feed::RssFeed};

#[function_component(RssFeedList)]
pub fn rss_feed_list() -> Html {
    let rss_feeds = use_state(|| vec![]);
    let display_modal = use_state(|| true);

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

    let onclick = {
        let display_modal = display_modal.clone();
        Callback::from(move |_| display_modal.set(true))
    };

    let onclose = {
        let display_modal = display_modal.clone();
        Callback::from(move |_| display_modal.set(false))
    };

    html! {
        <>
            <Modal display={*display_modal} {onclose}>
                <h1>{"Add RSS Feed"}</h1>

                <label for="name">{"Name"}</label>
                <input name="name" class={classes!(
                    "mb-5", "mt-2", "text-gray-600", "focus:outline-none", "focus:border",
                    "focus:border-indigo-700", "font-normal", "w-full", "h-10", "flex",
                    "items-center", "pl-3", "text-sm", "border-gray-300", "rounded", "border"
                )} placeholder={"Name"} />

                <label for="description">{"Description"}</label>
                <input name="description" class={classes!(
                    "mb-5", "mt-2", "text-gray-600", "focus:outline-none", "focus:border",
                    "focus:border-indigo-700", "font-normal", "w-full", "h-10", "flex",
                    "items-center", "pl-3", "text-sm", "border-gray-300", "rounded", "border"
                )} placeholder={"Description"} />

                <label for="url">{"URL"}</label>
                <input name="url" class={classes!(
                    "mb-5", "mt-2", "text-gray-600", "focus:outline-none", "focus:border",
                    "focus:border-indigo-700", "font-normal", "w-full", "h-10", "flex",
                    "items-center", "pl-3", "text-sm", "border-gray-300", "rounded", "border"
                )} placeholder={"URL"} />

                <div class={classes!("mb-4")}>
                    <input name="display_description" type="checkbox" class={classes!(
                        "appearance-none", "w-9", "focus:outline-none", "checked:bg-blue-300", "h-5",
                        "bg-gray-300", "rounded-full", "before:inline-block", "before:rounded-full",
                        "before:bg-blue-500", "before:h-4", "before:w-4", "checked:before:translate-x-full",
                        "shadow-inner", "transition-all", "duration-300", "before:ml-0.5",
                        "mr-4"
                    )} />
                    <label for="display_description">{"Display description underneath articles"}</label>
                </div>

                <div class={classes!("flex", "flex-row", "justify-end", "gap-1")}>
                    <button class={classes!(
                        "flex", "flex-row", "bg-gray-500", "hover:bg-gray-400", "transition-colors",
                        "py-2", "px-4", "rounded-xl", "font-semibold", "gap-1"
                    )}>
                        {"Cancel"}
                    </button>
                    <button class={classes!(
                        "flex", "flex-row", "bg-sky-500", "hover:bg-sky-400", "transition-colors",
                        "py-2", "px-4", "rounded-xl", "font-semibold", "gap-1"
                    )}>
                        {"Add"}
                    </button>
                </div>
            </Modal>
            <div class={classes!("flex", "flex-row", "justify-between", "items-center")}>
                <h2 class={classes!("flex-1", "text-xl")}>{"RSS Feeds"}</h2>
                <button {onclick} class={classes!(
                    "flex", "flex-row", "bg-sky-500", "hover:bg-sky-400", "transition-colors",
                    "py-2", "px-4", "rounded-xl", "font-semibold", "gap-1"
                )}>
                    <IconPlus />
                    {"Add"}
                </button>
            </div>
            { rss_feeds }
        </>
    }
}
