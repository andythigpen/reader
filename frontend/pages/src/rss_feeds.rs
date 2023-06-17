use yew::prelude::*;

use components::{
    footer::Footer, header::Header, page_container::PageContainer, page_content::PageContent,
    rss_feed_list::RssFeedList,
};

#[function_component(RssFeeds)]
pub fn rss_feeds() -> Html {
    html! {
        <PageContainer>
            <Header/>
            <PageContent>
                <RssFeedList />
            </PageContent>
            <Footer/>
        </PageContainer>
    }
}
