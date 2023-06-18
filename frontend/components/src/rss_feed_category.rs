use stores::category::CategoryStore;
use yew::prelude::*;
use yewdux::prelude::use_store_value;

use crate::{
    icons::{check::IconCheck, plus::IconPlus},
    list_item::ListItem,
};

pub enum CategoryAction {
    Add { id: String },
    Remove { id: String },
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: usize,

    #[prop_or_default]
    pub checked: bool,

    #[prop_or_default]
    pub onclick: Callback<CategoryAction>,
}

#[function_component(RssFeedCategory)]
pub fn rss_feed_category(props: &Props) -> Html {
    let store = use_store_value::<CategoryStore>();
    let model = &store.categories[props.id];

    let mut classes = classes!("cursor-pointer");

    let checked = props.checked;
    if checked {
        classes.push("dark:hover:!bg-teal-800");
        classes.push("dark:!bg-teal-900");
        classes.push("dark:text-white");
    } else {
        classes.push("dark:hover:bg-slate-700");
    }

    let onclick_item = {
        let id = model.id.clone();
        let onclick = props.onclick.clone();
        let checked = props.checked;
        Callback::from(move |_| {
            onclick.emit(if checked {
                CategoryAction::Remove { id: id.clone() }
            } else {
                CategoryAction::Add { id: id.clone() }
            })
        })
    };

    html! {
        <ListItem class={classes} onclick={onclick_item}>
            <div class={classes!("flex", "flex-col", "flex-1")}>
                <h2 class={classes!("dark:text-white", "text-lg")}>{model.name.clone()}</h2>
                <p>{model.description.clone()}</p>
            </div>
            <div class={classes!("flex", "flex-row", "items-center", "gap-4")}>
                if checked {
                    <IconCheck />
                } else {
                    <IconPlus />
                }
            </div>
        </ListItem>
    }
}
