use stores::article::ArticleStore;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    let (store, _) = use_store::<ArticleStore>();

    html! {
        <div class={classes!("dark:text-slate-500", "w-full", "max-w-4xl", "m-1", "text-center")}>
            if store.fetching {
                {"Loading..."}
            } else {
                {"You've reached the end"}
            }
        </div>
    }
}
