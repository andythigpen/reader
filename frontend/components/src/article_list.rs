use stores::article::ArticleStore;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{
    HtmlDivElement, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit,
};
use yew::prelude::*;
use yewdux::prelude::*;

use crate::article::Article;

#[function_component(ArticleList)]
pub fn article_list() -> Html {
    let store = use_store_value::<ArticleStore>();

    let node = use_node_ref();

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

    let articles = store
        .articles
        .iter()
        .enumerate()
        .map(|(id, _)| html! { <Article key={id} {id} /> })
        .collect::<Html>();

    html! {
        <>
            { articles }
            <div ref={node}></div>
        </>
    }
}
