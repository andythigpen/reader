use log::info;
use stores::article::ArticleStore;
use yew::prelude::*;
use yew_hooks::{use_window_scroll, use_window_size};
use yewdux::prelude::*;

use crate::article::Article;

#[function_component(ArticleList)]
pub fn article_list() -> Html {
    let (store, dispatch) = use_store::<ArticleStore>();

    let (x, y) = use_window_scroll();
    let (w, h) = use_window_size();

    use_effect_with_deps(
        move |_| {
            dispatch.reduce_mut(move |s| s.fetch());
        },
        (),
    );

    use_effect(move || {
        info!("x={} y={} w={} h={}", x, y, w, h);
    });

    html! {
        <div class={classes!("flex-grow", "flex", "flex-col", "max-w-4xl", "container",
                             "rounded-lg", "dark:bg-slate-800")}>
            {for store.articles.iter().enumerate().map(|(idx, _)| html! { <Article index={idx} /> })}
        </div>
    }
}
