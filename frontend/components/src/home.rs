use yew::prelude::*;

use crate::article_list::ArticleList;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class={classes!("flex", "flex-col", "justify-center", "items-center", "dark:bg-slate-900")}>
            <h1 class={classes!("dark:text-white")}>{"Reader"}</h1>
            <ArticleList/>
        </div>
    }
}
