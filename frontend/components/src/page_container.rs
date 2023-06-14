use yew::prelude::*;

use crate::header::Header;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(PageContainer)]
pub fn page_container(Props { children }: &Props) -> Html {
    let classes = classes!(
        "flex",
        "flex-col",
        "items-center",
        "dark:bg-gray-950",
        "min-h-screen",
        "dark:text-white",
    );
    html! {
        <div class={classes}>
            <Header/>
            { for children.iter() }
        </div>
    }
}
