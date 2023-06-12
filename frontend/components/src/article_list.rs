use stores::article::ArticleStore;
use wasm_bindgen::prelude::*;
use web_sys::{
    HtmlDivElement, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit,
};
use yew::prelude::*;
use yewdux::prelude::*;

use crate::article::Article;

#[function_component(ArticleList)]
pub fn article_list() -> Html {
    let (store, dispatch) = use_store::<ArticleStore>();

    let node = use_node_ref();

    use_effect_with_deps(
        {
            let node = node.clone();
            let dispatch = dispatch.clone();
            move |_| {
                let mut cb = None;
                if let Some(elem) = node.cast::<HtmlDivElement>() {
                    let callback = Closure::<
                        dyn Fn(Vec<IntersectionObserverEntry>, IntersectionObserver),
                    >::wrap(Box::new(
                        move |entries, _observer| {
                            if let Some(entry) = entries.first() {
                                if entry.is_intersecting() {
                                    dispatch.reduce_mut(|s| s.fetch());
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

    html! {
        <div class={classes!(
            "flex-grow", "flex", "flex-col", "max-w-4xl", "container", "rounded-lg", "dark:bg-slate-800"
        )}>
            {for store.articles.iter().enumerate().map(|(idx, _)| html! { <Article key={idx} index={idx} /> })}
            <div ref={node}></div>
        </div>
    }
}
