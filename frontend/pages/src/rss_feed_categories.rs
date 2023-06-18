use yew::prelude::*;

use components::{
    footer::Footer, header::Header, page_container::PageContainer, page_content::PageContent,
    rss_feed_category_list::RssFeedCategoryList,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component(RssFeedCategories)]
pub fn rss_feed_categories(Props { id }: &Props) -> Html {
    html! {
        <PageContainer>
            <Header/>

            <PageContent>
                <h2 class={classes!("flex-0", "text-xl")}>{"Select Categories"}</h2>
                <RssFeedCategoryList id={id.clone()} />
            </PageContent>

            <Footer/>
        </PageContainer>
    }
}
