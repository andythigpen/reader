use stores::article::ArticleStore;
use yew::prelude::*;
use yew::Properties;
use yewdux::prelude::*;

use crate::icons::arrow_top_right::IconArrowTopRight;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub index: usize,
}

#[function_component(Article)]
pub fn article(props: &Props) -> Html {
    let (store, _) = use_store::<ArticleStore>();
    let article = &store.articles[props.index];
    html! {
        <article class={classes!("flex", "flex-row", "m-2", "dark:text-slate-400", "items-center")}>
            <a href={article.url.clone()} class={classes!("flex-1")}>
                <h2 class={classes!("text-lg", "dark:text-white")}>
                    {article.title.clone()}
                </h2>
                <p>{article.description.clone()}</p>
            </a>
            <a href={article.url.clone()} class={classes!("flex-0")} target={"_blank"} rel={"noopener noreferrer"}>
                <IconArrowTopRight/>
            </a>
        </article>
    }
}
