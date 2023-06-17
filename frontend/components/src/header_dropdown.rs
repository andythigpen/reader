use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub display: bool,
}

#[function_component(HeaderDropdown)]
pub fn header_dropdown(
    Props {
        children,
        class,
        display,
    }: &Props,
) -> Html {
    let mut classes = classes!(
        "fixed",
        "top-10",
        "z-10",
        "bg-white",
        "divide-y",
        "divide-gray-100",
        "rounded-b-lg",
        "shadow-2xl",
        "w-full",
        "sm:w-64",
        "dark:bg-slate-950",
        class.clone()
    );
    if !display {
        classes.push("hidden");
    }
    html! {
        <div class={classes}>
            <ul class={classes!("py-2", "text-gray-700", "dark:text-gray-200")}>
                {for children.iter()}
            </ul>
        </div>
    }
}
