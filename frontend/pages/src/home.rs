use stores::rss_feed::RssFeedStore;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, NavigationType, PerformanceNavigationTiming};
use yew::prelude::*;
use yewdux::prelude::*;

use components::{
    article_list::ArticleList, footer::Footer, header::Header,
    icons::chevron_down::IconChevronDown, page_container::PageContainer, page_content::PageContent,
};
use stores::article::ArticleStore;

#[function_component(Home)]
pub fn home() -> Html {
    let fetching = use_selector(|s: &ArticleStore| s.fetching);

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
            })
        },
        (),
    );

    html! {
        <PageContainer>
            <Header>
                <div class={classes!("flex", "flex-row", "flex-1", "gap-1")}>
                    {"All"}
                    <IconChevronDown/>
                </div>
                <div class="hidden fixed sm:absolute left-0 sm:left-auto top-6 z-10 bg-white divide-y divide-gray-100 rounded-b-lg shadow-2xl w-full sm:w-64 dark:bg-slate-950">
                    <ul class="py-2 text-gray-700 dark:text-gray-200">
                        <li>
                            <a href="#" class="block px-8 sm:px-4 py-4 hover:bg-slate-100 dark:hover:bg-slate-600 dark:hover:text-white">{"All"}</a>
                        </li>
                        <li>
                            <a href="#" class="block px-8 sm:px-4 py-4 hover:bg-slate-100 dark:hover:bg-slate-600 dark:hover:text-white">{"Technology"}</a>
                        </li>
                        <li>
                            <a href="#" class="block px-8 sm:px-4 py-4 hover:bg-slate-100 dark:hover:bg-slate-600 dark:hover:text-white">{"News"}</a>
                        </li>
                        <li>
                            <a href="#" class="block px-8 sm:px-4 py-4 hover:bg-slate-100 dark:hover:bg-slate-600 dark:hover:text-white">{"Sports"}</a>
                        </li>
                    </ul>
                </div>
            </Header>

            <PageContent>
                <ArticleList/>
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
