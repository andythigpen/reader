use yew::prelude::*;

use components::{
    category_list::CategoryList, footer::Footer, header::Header, page_container::PageContainer,
    page_content::PageContent,
};

#[function_component(Categories)]
pub fn categories() -> Html {
    html! {
        <PageContainer>
            <Header/>
            <PageContent>
                <CategoryList />
            </PageContent>
            <Footer/>
        </PageContainer>
    }
}
