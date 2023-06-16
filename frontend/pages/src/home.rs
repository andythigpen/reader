use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, NavigationType, PerformanceNavigationTiming};
use yew::prelude::*;
use yewdux::prelude::*;

use components::article_list::ArticleList;
use components::footer::Footer;
use components::page_container::PageContainer;
use components::page_content::PageContent;
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

    html! {
        <PageContainer>
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
