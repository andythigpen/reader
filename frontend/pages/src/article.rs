use yew::prelude::*;

use components::{
    footer::Footer, header::Header, page_container::PageContainer, page_content::PageContent,
    readability_article::ReadabilityArticle,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component(Article)]
pub fn article(props: &Props) -> Html {
    let id = props.id.clone();
    let fallback = html! {<div>{"Loading..."}</div>};
    html! {
        <PageContainer>
            <Header/>
            <PageContent>
                <Suspense {fallback}>
                    <ReadabilityArticle {id} />
                </Suspense>
            </PageContent>
            <Footer/>
        </PageContainer>
    }
}
