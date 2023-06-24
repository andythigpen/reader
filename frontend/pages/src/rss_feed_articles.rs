use hooks::use_page_reload;
use stores::rss_feed::RssFeedStore;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;

use components::{
    article_list::ArticleList, footer::Footer, header::Header, page_container::PageContainer,
    page_content::PageContent,
};
use stores::article::ArticleStore;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub id: Option<String>,
}

#[function_component(RssFeedArticles)]
pub fn rss_feed_articles(props: &Props) -> Html {
    let fetching = use_selector(|s: &ArticleStore| s.fetching);
    let rss_feed_id = props.id.clone();

    // reload the article store when the browser reloads the page
    use_page_reload(|| {
        spawn_local(async move {
            Dispatch::<ArticleStore>::new()
                .reduce_mut_future(|s| Box::pin(async move { s.reload().await }))
                .await
        });
    });

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
            <Header />

            <PageContent>
                <ArticleList {rss_feed_id} />
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
