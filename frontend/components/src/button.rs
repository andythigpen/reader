use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub primary: bool,

    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,

    #[prop_or_default]
    pub children: Children,
}

#[function_component(Button)]
pub fn button(
    Props {
        primary,
        onclick,
        children,
    }: &Props,
) -> Html {
    let mut classes = classes!(
        "flex",
        "flex-row",
        "transition-colors",
        "py-2",
        "px-4",
        "rounded-xl",
        "font-semibold",
        "gap-1"
    );
    if *primary {
        classes.push("bg-sky-500");
        classes.push("hover:bg-sky-400");
    } else {
        classes.push("bg-gray-500");
        classes.push("hover:bg-gray-400");
    }
    html! {
        <button onclick={onclick} class={classes}>
            {for children.iter()}
        </button>
    }
}
