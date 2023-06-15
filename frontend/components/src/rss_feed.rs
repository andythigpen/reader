use entity::rss_feed::Model;
use yew::prelude::*;
use yew::Properties;

use crate::icons::pencil_square::IconPencilSquare;
use crate::icons::trash::IconTrash;
use crate::list_item::ListItem;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub model: Model,
}

#[function_component(RssFeed)]
pub fn rss_feed(props: &Props) -> Html {
    let model = props.model.clone();
    html! {
        <ListItem>
            <div class={classes!("flex", "flex-col", "flex-1")}>
                <h2 class={classes!("dark:text-white", "text-lg")}>{model.name}</h2>
                <span>{model.created_at}</span>
                <p>{model.description}</p>
                <p>{model.url}</p>
            </div>
            <IconPencilSquare class={classes!("cursor-pointer", "mx-2")} />
            <IconTrash class={classes!("cursor-pointer", "mx-2")} />
        </ListItem>
    }
}
