use log::info;
use stores::article::ArticleStore;
use web_sys::HtmlDivElement;
use yew::prelude::*;
use yew_hooks::{use_debounce, use_window_scroll, use_window_size};
use yewdux::prelude::*;

use crate::article::Article;

#[function_component(ArticleList)]
pub fn article_list() -> Html {
    let (store, dispatch) = use_store::<ArticleStore>();

    let (x, y) = use_window_scroll();
    let (w, h) = use_window_size();
    let node = use_node_ref();

    {
        let dispatch = dispatch.clone();
        use_effect_with_deps(
            move |_| {
                dispatch.reduce_mut(move |s| s.fetch());
            },
            (),
        );
    }

    let debouce = {
        let dispatch = dispatch.clone();
        use_debounce(
            move || {
                info!("should scroll debounced");
                dispatch.reduce_mut(|s| s.fetch());
            },
            500,
        )
    };

    {
        let node = node.clone();
        use_effect(move || {
            let div = node.cast::<HtmlDivElement>().unwrap();
            let offset_height = div.offset_height();
            info!("x={} y={} w={} h={} offseth={}", x, y, w, h, offset_height);
            if offset_height - (y as i32 + h as i32) < 150 {
                info!("should scroll");
                debouce.run();
            }
        });
    }

    html! {
        <div ref={node} class={classes!(
            "flex-grow", "flex", "flex-col", "max-w-4xl", "container", "rounded-lg", "dark:bg-slate-800"
        )}>
            {for store.articles.iter().enumerate().map(|(idx, _)| html! { <Article index={idx} /> })}
        </div>
    }
}
