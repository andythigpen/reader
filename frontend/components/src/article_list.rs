use stores::article::ArticleStore;
use yew::prelude::*;
use yew_hooks::{use_window_scroll, use_window_size};
use yewdux::prelude::*;

use crate::article::Article;

#[function_component(ArticleList)]
pub fn article_list() -> Html {
    let loaded = use_state(|| false);
    let (store, dispatch) = use_store::<ArticleStore>();

    let (x, y) = use_window_scroll();
    let (w, h) = use_window_size();

    use_effect(move || {
        if !*loaded {
            dispatch.reduce_mut(move |s| s.fetch());
            loaded.set(true);
        }
    });

    html! {
        <div class={classes!("flex", "flex-col", "max-w-4xl", "container",
                             "rounded-lg", "dark:bg-slate-800")}>
            <p class={classes!("fixed")}>{x}{","}{y}{" - "}{w}{"x"}{h}</p>
            {for store.articles.iter().enumerate().map(|(idx, _)| html! { <Article index={idx} /> })}
        </div>
    }
}
