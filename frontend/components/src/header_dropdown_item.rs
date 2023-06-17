use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub class: Classes,
}

#[function_component(HeaderDropdownItem)]
pub fn header_dropdown_item(Props { children, class }: &Props) -> Html {
    let classes = classes!(
        "hover:bg-slate-100",
        "dark:hover:bg-slate-600",
        "dark:hover:text-white",
        class.clone()
    );
    html! {
        <li class={classes}>
            { for children.iter() }
        </li>
    }
}
