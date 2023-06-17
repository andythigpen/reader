use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ListItem)]
pub fn list_item(Props { children }: &Props) -> Html {
    let classes = classes!(
        "flex",
        "flex-row",
        "my-2",
        "dark:text-slate-400",
        "dark:bg-slate-800",
        // "items-center",
        "justify-between",
        "w-full",
        "rounded-2xl",
        "p-3",
    );
    html! {
        <div class={classes}>
            { for children.iter() }
        </div>
    }
}
