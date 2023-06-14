use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(PageContent)]
pub fn page_content(Props { children }: &Props) -> Html {
    let classes = classes!(
        "flex-grow",
        "flex",
        "flex-col",
        "w-full",
        "lg:max-w-5xl",
        "rounded-b-lg",
        "dark:bg-slate-900",
        "px-4",
        "py-2",
    );
    html! {
        <div class={classes}>
            { for children.iter() }
        </div>
    }
}
