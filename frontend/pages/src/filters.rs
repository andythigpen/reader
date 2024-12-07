use yew::prelude::*;

use components::{
    filter_list::FilterList, footer::Footer, header::Header, page_container::PageContainer,
    page_content::PageContent,
};

#[function_component(Filters)]
pub fn filters() -> Html {
    html! {
        <PageContainer>
            <Header/>
            <PageContent>
                <FilterList />
            </PageContent>
            <Footer/>
        </PageContainer>
    }
}
