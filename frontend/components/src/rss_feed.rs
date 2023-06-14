use entity::rss_feed::Model;
use yew::prelude::*;
use yew::Properties;

use crate::icons::pencil_square::IconPencilSquare;
use crate::icons::trash::IconTrash;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub model: Model,
}

#[function_component(RssFeed)]
pub fn rss_feed(props: &Props) -> Html {
    let model = props.model.clone();

    let classes = classes!(
        "flex",
        "flex-row",
        "my-2",
        "dark:text-slate-400",
        "dark:bg-slate-800",
        "items-center",
        "justify-between",
        "w-full",
        "rounded-2xl",
        "p-3",
    );
    html! {
        <div class={classes}>
            <div class={classes!("flex", "flex-col", "flex-1")}>
                <h2 class={classes!("dark:text-white", "text-lg")}>{model.name}</h2>
                <span>{model.created_at}</span>
                <p>{model.description}</p>
                <p>{model.url}</p>
            </div>
            <IconPencilSquare class={classes!("cursor-pointer", "mx-2")} />
            <IconTrash class={classes!("cursor-pointer", "mx-2")} />
        </div>
    }
}
