use entity::category::Model as Category;
use gloo_net::http::Request;
use stores::category::CategoryStore;
use stores::rss_feed::RssFeedStore;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::list_item::ListItem;
use crate::rss_feed_category::{CategoryAction, RssFeedCategory};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component(RssFeedCategoryList)]
pub fn rss_feed_category_list(Props { id }: &Props) -> Html {
    let rss_feed = {
        let id = id.clone();
        use_selector(move |s: &RssFeedStore| s.get_by_id(&id))
    };

    let categories = use_selector(|s: &CategoryStore| s.categories.clone());

    let current = use_state(|| vec![]);

    {
        let rss_feed = rss_feed.clone();
        let current = current.clone();
        let id = id.clone();
        use_effect_with_deps(
            move |_| {
                let rss_feed = rss_feed.clone();
                spawn_local(async move {
                    if rss_feed.is_none() {
                        Dispatch::<RssFeedStore>::new()
                            .reduce_mut_future(|s| Box::pin(async move { s.fetch().await }))
                            .await;
                    }
                    Dispatch::<CategoryStore>::new()
                        .reduce_mut_future(|s| Box::pin(async move { s.fetch().await }))
                        .await;

                    let resp: Vec<Category> =
                        Request::get(&format!("/api/rss_feeds/{}/categories", id))
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    current.set(resp.iter().map(|c| c.id.to_owned()).collect());
                });
                || ()
            },
            (),
        );
    }

    let onclick_item = {
        let current = current.clone();
        let rss_feed_id = (*rss_feed).clone().map_or("".to_string(), |r| r.id);
        Callback::from(move |action| {
            let current = current.clone();
            let rss_feed_id = rss_feed_id.clone();
            spawn_local(async move {
                match action {
                    CategoryAction::Add { id } => {
                        let resp = Request::post(&format!(
                            "/api/rss_feeds/{}/categories/{}",
                            rss_feed_id, id
                        ))
                        .send()
                        .await
                        .unwrap();
                        if resp.ok() {
                            let mut c = (*current).clone();
                            c.push(id);
                            current.set(c);
                        }
                    }
                    CategoryAction::Remove { id } => {
                        let resp = Request::delete(&format!(
                            "/api/rss_feeds/{}/categories/{}",
                            rss_feed_id, id
                        ))
                        .send()
                        .await
                        .unwrap();
                        if resp.ok() {
                            let mut c = (*current).clone();
                            c.retain(|i| *i != id);
                            current.set(c);
                        }
                    }
                }
            })
        })
    };

    let categories_list = use_state(|| html! {});

    {
        let onclick_item = onclick_item.clone();
        let categories_list = categories_list.clone();
        use_effect_with_deps(
            move |(categories, current)| {
                categories_list.set(if categories.is_empty() {
                    html! {
                        <ListItem>{"No categories."}</ListItem>
                    }
                } else {
                    categories
                        .iter()
                        .enumerate()
                        .map(|(id, model)| {
                            let checked = current.iter().find(|id| **id == model.id).is_some();
                            html! { <RssFeedCategory key={id} {id} {checked} onclick={onclick_item.clone()} /> }
                        })
                        .collect::<Html>()
                });
            },
            (categories, current),
        );
    }

    // let categories = if categories.is_empty() {
    //     html! {
    //         <ListItem>{"No categories."}</ListItem>
    //     }
    // } else {
    //     categories
    //         .iter()
    //         .enumerate()
    //         .map(|(id, model)| {
    //             let checked = current.iter().find(|id| **id == model.id).is_some();
    //             html! { <RssFeedCategory key={id} {id} {checked} onclick={onclick_item.clone()} /> }
    //         })
    //         .collect::<Html>()
    // };

    // let categories = if categories.is_empty() {
    //     html! {
    //         <ListItem>{"No categories."}</ListItem>
    //     }
    // } else {
    //     categories
    //         .iter()
    //         .enumerate()
    //         .map(|(id, model)| {
    //             let checked = current.iter().find(|id| **id == model.id).is_some();
    //             html! { <RssFeedCategory key={id} {id} {checked} onclick={onclick_item.clone()} /> }
    //         })
    //         .collect::<Html>()
    // };

    html! {
        <>
            <h3 class={classes!("flex-0", "text-md")}>{(*rss_feed).clone().map_or("".to_string(), |r| r.name)}</h3>
            { (*categories_list).clone() }
        </>
    }
}
