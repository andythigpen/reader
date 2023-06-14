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
