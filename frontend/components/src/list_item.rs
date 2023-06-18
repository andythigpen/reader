use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(ListItem)]
pub fn list_item(
    Props {
        children,
        class,
        onclick,
    }: &Props,
) -> Html {
    let classes = classes!(
        "flex",
        "flex-row",
        "my-2",
        "dark:text-slate-400",
        "dark:bg-slate-800",
        "justify-between",
        "w-full",
        "rounded-2xl",
        "p-3",
        class.clone()
    );
    html! {
        <div class={classes} {onclick}>
            { for children.iter() }
        </div>
    }
}
