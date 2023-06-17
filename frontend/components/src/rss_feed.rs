use gloo_net::http::Request;
use router::Route;
use stores::rss_feed::RssFeedStore;
use time::{format_description, format_description::well_known::Iso8601, OffsetDateTime};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::Properties;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::button::Button;
use crate::icons::pencil_square::IconPencilSquare;
use crate::icons::tag::IconTag;
use crate::icons::trash::IconTrash;
use crate::list_item::ListItem;
use crate::list_item_thumb::ListItemThumb;
use crate::modal::Modal;
use crate::rss_feed_form::ModalAction;
use crate::rss_feed_form::RssFeedForm;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub id: usize,
}

#[function_component(RssFeed)]
pub fn rss_feed(&Props { id }: &Props) -> Html {
    let display_edit_modal = use_state(|| false);
    let display_delete_modal = use_state(|| false);
    let store = use_store_value::<RssFeedStore>();
    let model = &store.rss_feeds[id];

    let edit = {
        let display_edit_modal = display_edit_modal.clone();
        Callback::from(move |_| display_edit_modal.set(true))
    };

    let close_edit_modal = {
        let display_edit_modal = display_edit_modal.clone();
        Callback::from(move |_| display_edit_modal.set(false))
    };

    let close_form = {
        let display_edit_modal = display_edit_modal.clone();
        Callback::from(move |action| match action {
            ModalAction::Close => display_edit_modal.set(false),
            ModalAction::Confirm(model) => {
                let display_edit_modal = display_edit_modal.clone();
                spawn_local(async move {
                    Request::put(&format!("/api/rss_feeds/{}", model.id))
                        .json(&model)
                        .unwrap()
                        .send()
                        .await
                        .unwrap();
                    Dispatch::<RssFeedStore>::new().reduce_mut(|s| s.rss_feeds[id] = model);
                    display_edit_modal.set(false);
                });
            }
        })
    };

    let delete = {
        let display_delete_modal = display_delete_modal.clone();
        Callback::from(move |_| display_delete_modal.set(true))
    };

    let close_delete_modal = {
        let display_delete_modal = display_delete_modal.clone();
        Callback::from(move |_| display_delete_modal.set(false))
    };

    let delete_confirm = {
        let model_id = model.id.clone();
        let display_delete_modal = display_delete_modal.clone();
        Callback::from(move |_| {
            let model_id = model_id.clone();
            let display_delete_modal = display_delete_modal.clone();
            spawn_local(async move {
                Request::delete(&format!("/api/rss_feeds/{}", model_id))
                    .send()
                    .await
                    .unwrap();
                Dispatch::<RssFeedStore>::new().reduce_mut(|s| s.rss_feeds.remove(id));
                display_delete_modal.set(false);
            });
        })
    };

    let format = format_description::parse("[year]-[month]-[day]").unwrap();
    let created_at = OffsetDateTime::parse(&model.created_at, &Iso8601::DEFAULT)
        .unwrap()
        .format(&format)
        .unwrap();

    html! {
        <ListItem>
            <Modal display={*display_edit_modal} onclose={close_edit_modal}>
                <RssFeedForm model={model.to_owned()} onclose={close_form} />
            </Modal>
            <Modal display={*display_delete_modal} onclose={close_delete_modal.clone()}>
                <h1 class={classes!("text-xl", "mb-4")}>{"Delete RSS Feed"}</h1>

                <p class={classes!("m-4")}>{model.name.clone()}</p>

                <div class={classes!("flex", "flex-row", "justify-end", "gap-1")}>
                    <Button onclick={close_delete_modal}>{"Cancel"}</Button>
                    <Button onclick={delete_confirm} primary=true>{"Delete"}</Button>
                </div>
            </Modal>
            <ListItemThumb text={model.abbreviation.to_uppercase()} color={model.color.clone()} />
            <div class={classes!("flex", "flex-col", "flex-1")}>
                <h2 class={classes!("dark:text-white", "text-lg")}>{model.name.clone()}</h2>
                <span class={classes!("text-sm")}>{"Created on "}{created_at}</span>
                <p>{model.description.clone()}</p>
            </div>
            <div class={classes!("flex", "flex-row", "items-center", "gap-4")}>
                <Link<Route> to={Route::RssFeedCategories{ id: model.id.clone() }}>
                    <IconTag class={classes!("cursor-pointer")} />
                </Link<Route>>
                <a onclick={edit}>
                    <IconPencilSquare class={classes!("cursor-pointer")} />
                </a>
                <a onclick={delete}>
                    <IconTrash class={classes!("cursor-pointer")} />
                </a>
            </div>
        </ListItem>
    }
}
