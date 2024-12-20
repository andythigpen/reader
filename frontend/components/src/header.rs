use gloo_net::http::Request;
use router::Route;
use stores::article::ArticleStore;
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::header_dropdown::HeaderDropdown;
use crate::header_dropdown_item::HeaderDropdownItem;
use crate::icons::arrow_path::IconArrowPath;
use crate::icons::funnel::IconFunnel;
use crate::icons::tag::IconTag;
use crate::icons::x_mark::IconXMark;
use crate::icons::{bars_3::IconBars3, rss::IconRss};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Header)]
pub fn header(Props { children }: &Props) -> Html {
    let display_menu = use_state(|| false);
    let refreshing = use_state(|| false);

    let onclick_refresh = {
        let display_menu = display_menu.clone();
        let refreshing = refreshing.clone();
        Dispatch::<ArticleStore>::new().reduce_mut_future_callback(move |state| {
            let display_menu = display_menu.clone();
            let refreshing = refreshing.clone();
            Box::pin(async move {
                display_menu.set(false);
                refreshing.set(true);
                let resp = Request::post("/api/rss_feeds/fetch").send().await.unwrap();
                if resp.ok() {
                    state.reload().await;
                    window().unwrap().scroll_with_x_and_y(0.0, 0.0);
                }
                refreshing.set(false);
            })
        })
    };

    let onclick_menu = {
        let display_menu = display_menu.clone();
        Callback::from(move |_| display_menu.set(!*display_menu))
    };

    let classes = classes!(
        "flex",
        "flex-row",
        "dark:text-white",
        "dark:bg-slate-950",
        "w-full",
        "max-w-5xl",
        "p-1",
        "z-10",
        "leading-10",
        "items-center",
        "sticky",
        "top-0",
        "relative",
        // inverted border below
        "before:content-['']",
        "before:bg-transparent",
        "before:rounded-t-lg",
        "before:absolute",
        "before:max-w-5xl",
        "before:w-full",
        "before:inset-x-0",
        "before:-bottom-6",
        "before:h-6",
        "before:shadow-[0px_-5px_0px_0px]",
        "before:shadow-slate-950",
    );
    html! {
        <div class={classes}>
            <div class={classes!("flex-0")}>
                <Link<Route> to={Route::Home}>
                    <IconRss class={classes!("inline", "mx-2")}/>
                    {"Reader"}
                </Link<Route>>
            </div>
            <div class={classes!("flex", "flex-col", "flex-1", "relative", "items-center")}>
                {for children.iter()}
            </div>
            <div class={classes!("relative", "flex", "flex-row", "items-center", "gap-2")}>
                if *refreshing {
                    <IconArrowPath class={classes!("animate-spin")} />
                }
                <div onclick={onclick_menu} class={classes!("flex", "flex-col", "h-10", "justify-center")}>
                    if !*display_menu {
                        <IconBars3 class={classes!("mx-2", "cursor-pointer")}/>
                    } else {
                        <IconXMark class={classes!("mx-2", "cursor-pointer")}/>
                    }
                </div>
                <HeaderDropdown display={*display_menu} class={classes!("right-0", "sm:-right-1")}>
                    <HeaderDropdownItem>
                        <div onclick={onclick_refresh} class={classes!(
                            "block", "px-8", "sm:px-4", "py-4", "flex", "flex-row", "gap-2", "cursor-pointer", "items-center"
                        )}>
                            <IconArrowPath />
                            {"Refresh Feeds"}
                        </div>
                    </HeaderDropdownItem>
                    <HeaderDropdownItem>
                        <Link<Route> to={Route::Categories} classes={classes!(
                            "block", "px-8", "sm:px-4", "py-4", "flex", "flex-row", "gap-2", "items-center"
                        )}>
                            <IconTag />
                            {"Categories"}
                        </Link<Route>>
                    </HeaderDropdownItem>
                    <HeaderDropdownItem>
                        <Link<Route> to={Route::Filters} classes={classes!(
                            "block", "px-8", "sm:px-4", "py-4", "flex", "flex-row", "gap-2", "items-center"
                        )}>
                            <IconFunnel />
                            {"Filters"}
                        </Link<Route>>
                    </HeaderDropdownItem>
                    <HeaderDropdownItem>
                        <Link<Route> to={Route::RssFeeds} classes={classes!(
                            "block", "px-8", "sm:px-4", "py-4", "flex", "flex-row", "gap-2", "items-center"
                        )}>
                            <IconRss />
                            {"RSS Feeds"}
                        </Link<Route>>
                    </HeaderDropdownItem>
                </HeaderDropdown>
            </div>
        </div>
    }
}
