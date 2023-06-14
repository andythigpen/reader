use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Footer)]
pub fn footer(Props { children }: &Props) -> Html {
    html! {
        <div class={classes!("dark:text-slate-500", "w-full", "max-w-4xl", "m-1", "text-center")}>
            { for children.iter() }
        </div>
    }
}
