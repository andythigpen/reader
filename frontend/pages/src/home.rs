use router::Route;
use stores::{category::CategoryStore, rss_feed::RssFeedStore};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, NavigationType, PerformanceNavigationTiming};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use components::{
    article_list::ArticleList, footer::Footer, header::Header, header_dropdown::HeaderDropdown,
    header_dropdown_item::HeaderDropdownItem, icons::chevron_down::IconChevronDown,
    page_container::PageContainer, page_content::PageContent,
};
use stores::article::ArticleStore;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub category_id: Option<String>,
}

#[function_component(Home)]
pub fn home(props: &Props) -> Html {
    let fetching = use_selector(|s: &ArticleStore| s.fetching);
    let display_categories = use_state(|| false);
    let category_id = props.category_id.clone();
    let category_name = use_state(|| None);
    let categories = use_selector(|s: &CategoryStore| s.categories.clone());

    let route = use_route::<Route>();

    // reload the article store when the browser reloads the page
    use_effect_with_deps(
        {
            move |_| {
                let arr = window()
                    .unwrap()
                    .performance()
                    .unwrap()
                    .get_entries_by_type("navigation");
                for elem in arr {
                    let entry: PerformanceNavigationTiming = elem.unchecked_into();
                    if entry.type_() == NavigationType::Reload {
                        spawn_local(async move {
                            Dispatch::<ArticleStore>::new()
                                .reduce_mut_future(|s| Box::pin(async move { s.reload().await }))
                                .await;
                        });
                    }
                }
                || ()
            }
        },
        (),
    );

    use_effect_with_deps(
        |_| {
            spawn_local(async move {
                Dispatch::<RssFeedStore>::new()
                    .reduce_mut_future(|s| Box::pin(async move { s.fetch().await }))
                    .await;
                Dispatch::<CategoryStore>::new()
                    .reduce_mut_future(|s| Box::pin(async move { s.fetch().await }))
                    .await;
            })
        },
        (),
    );

    let menu = categories
        .iter()
        .map(|c| {
            html! {
                <HeaderDropdownItem>
                    <Link<Route> to={Route::CategoryArticles{ id: c.id.clone() }} classes={classes!("block", "px-8", "sm:px-4", "py-4")}>
                        {c.name.clone()}
                    </Link<Route>>
                </HeaderDropdownItem>
            }
        })
        .collect::<Html>();

    let onclick_category = {
        let display_categories = display_categories.clone();
        Callback::from(move |_| display_categories.set(!*display_categories))
    };
    {
        let display_categories = display_categories.clone();
        let category_id = category_id.clone();
        let category_name = category_name.clone();
        use_effect_with_deps(
            move |_| {
                display_categories.set(false);
                Dispatch::<CategoryStore>::new().reduce_mut(|s| {
                    let category = s.get_by_id(&category_id.unwrap_or("".to_string()));
                    category_name.set(category.map_or(None, |c| Some(c.name.to_owned())));
                });
            },
            (route, categories),
        );
    }

    html! {
        <PageContainer>
            <Header>
                <div onclick={onclick_category} class={classes!(
                    "flex", "flex-row", "flex-1", "gap-1", "cursor-pointer", "items-center"
                )}>
                    {(*category_name).clone().unwrap_or("All".to_string())}
                    <IconChevronDown/>
                </div>
                <HeaderDropdown display={*display_categories} class={classes!(
                    "left-0", "sm:left-auto"
                )}>
                    <HeaderDropdownItem>
                        <Link<Route> to={Route::Home} classes={classes!("block", "px-8", "sm:px-4", "py-4")}>{"All"}</Link<Route>>
                    </HeaderDropdownItem>
                    {menu}
                </HeaderDropdown>
            </Header>

            <PageContent>
                <ArticleList {category_id} />
            </PageContent>

            <Footer>
            if *fetching {
                {"Loading..."}
            } else {
                {"You've reached the end"}
            }
            </Footer>
        </PageContainer>
    }
}
