use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(PageContent)]
pub fn page_content(Props { children }: &Props) -> Html {
    html! {
        <div class={classes!(
            "flex-grow", "flex", "flex-col", "max-w-4xl", "container", "rounded-lg", "dark:bg-slate-800", "px-4", "py-2"
        )}>
            { for children.iter() }
        </div>
    }
}
