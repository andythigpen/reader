use router::Route;
use stores::article::ArticleStore;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{
    HtmlDivElement, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit,
};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::article::Article;
use crate::list_item::ListItem;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub category_id: Option<String>,
}

#[function_component(ArticleList)]
pub fn article_list(props: &Props) -> Html {
    let (store, dispatch) = use_store::<ArticleStore>();
    dispatch.reduce_mut(|s| s.category_id(props.category_id.clone()));

    let route = use_route::<Route>();

    let node = use_node_ref();

    use_effect_with_deps(
        |_| {
            spawn_local(async move {
                Dispatch::<ArticleStore>::new()
                    .reduce_mut_future(|s| Box::pin(async move { s.fetch().await }))
                    .await;
            });
            || ()
        },
        route,
    );

    use_effect_with_deps(
        {
            let node = node.clone();
            move |_| {
                let mut cb = None;
                if let Some(elem) = node.cast::<HtmlDivElement>() {
                    let callback = Closure::<
                        dyn Fn(Vec<IntersectionObserverEntry>, IntersectionObserver),
                    >::wrap(Box::new(
                        move |entries, _observer| {
                            if let Some(entry) = entries.first() {
                                if entry.is_intersecting() {
                                    spawn_local(async move {
                                        Dispatch::<ArticleStore>::new()
                                            .reduce_mut_future(|s| {
                                                Box::pin(async move { s.fetch().await })
                                            })
                                            .await;
                                    });
                                }
                            }
                        },
                    ));
                    if let Ok(o) = IntersectionObserver::new_with_options(
                        callback.as_ref().dyn_ref().unwrap(),
                        IntersectionObserverInit::new().root_margin("200px"),
                    ) {
                        o.observe(&elem);
                        cb = Some(callback);
                    }
                }
                move || drop(cb)
            }
        },
        node.clone(),
    );

    let articles = if store.articles.is_empty() {
        html! {
            <ListItem>{"No articles found."}</ListItem>
        }
    } else {
        store
            .articles
            .iter()
            .enumerate()
            .map(|(id, _)| html! { <Article key={id} {id} /> })
            .collect::<Html>()
    };

    html! {
        <>
            { articles }
            <div ref={node}></div>
        </>
    }
}
