use stores::rss_feed::RssFeedStore;
use yew::prelude::*;

use components::{
    footer::Footer, header::Header, page_container::PageContainer, page_content::PageContent,
};
use yewdux::prelude::use_store_value;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component(RssFeedCategories)]
pub fn rss_feed_categories(Props { id }: &Props) -> Html {
    let rss_feed_store = use_store_value::<RssFeedStore>();
    let rss_feed = rss_feed_store.get_by_id(&id).unwrap(); // FIXME: this can be None on a refresh of the page

    html! {
        <PageContainer>
            <Header/>

            <PageContent>
                <h2>{"Select Categories"}</h2>
                <h3>{rss_feed.name.clone()}</h3>

            </PageContent>

            <Footer/>
        </PageContainer>
    }
}
