use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <div class={classes!("dark:text-white", "w-full", "max-w-4xl", "m-1")}>
            {"Reader"}
        </div>
    }
}
