use yew::prelude::*;

use crate::header::Header;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(PageContainer)]
pub fn page_container(Props { children }: &Props) -> Html {
    html! {
        <div class={classes!("flex", "flex-col", "items-center", "dark:bg-slate-900", "min-h-screen", "dark:text-white")}>
            <Header/>
            { for children.iter() }
        </div>
    }
}
