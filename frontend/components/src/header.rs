use yew::prelude::*;

use crate::icons::{arrow_path::IconArrowPath, rss::IconRss};

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <div class={classes!("flex", "flex-row", "dark:text-white", "w-full", "max-w-4xl", "m-1", "items-center")}>
            <IconRss class={classes!("inline", "mx-2")}/>
            <div class={classes!("flex-1")}>
                {"Reader"}
            </div>
            <IconArrowPath class={classes!("mx-2")}/>
        </div>
    }
}
