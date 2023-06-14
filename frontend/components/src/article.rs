use stores::article::ArticleStore;
use yew::prelude::*;
use yew::Properties;
use yewdux::prelude::*;

use crate::icons::arrow_top_right::IconArrowTopRight;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub id: usize,
}

#[function_component(Article)]
pub fn article(&Props { id }: &Props) -> Html {
    let store = use_store_value::<ArticleStore>();
    let article = &store.articles[id];
    let classes = classes!(
        "flex",
        "flex-row",
        "my-2",
        "dark:text-slate-400",
        "dark:dark:bg-slate-800",
        "items-center",
        "justify-between",
        "w-full",
        "rounded-2xl",
        "p-3",
    );
    html! {
        <article class={classes}>
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
