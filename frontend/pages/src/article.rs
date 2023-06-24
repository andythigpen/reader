use components::{
    date::Date, footer::Footer, header::Header, icons::loading::IconLoading,
    page_container::PageContainer, page_content::PageContent,
    readability_article::ReadabilityArticle,
};
use stores::article::ArticleStore;
use yew::prelude::*;
use yewdux::prelude::use_store_value;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[function_component(Article)]
pub fn article(props: &Props) -> Html {
    let article_store = use_store_value::<ArticleStore>();
    let id = props.id.clone();
    let fallback = {
        let article = article_store.by_article_id(&props.id);
        let title = article.as_ref().map(|a| a.title.clone());
        let pub_date = article.as_ref().map(|a| a.pub_date.clone());
        let url = article.as_ref().map(|a| a.url.clone());
        html! {
            <div class={classes!("flex", "flex-col", "items-center", "m-2", "md:my-4")}>
                <h1 class={classes!("md:max-w-2xl", "text-3xl")}>{title}</h1>
                if pub_date.is_some() {
                    <aside class={classes!("text-sm", "my-4")}>
                        {"Published at "}<Date value={pub_date.unwrap()}/>
                        {" | "}
                        <a href={url} class={classes!("text-sky-400")}>{"Original Article"}</a>
                    </aside>
                }
                <div class={classes!("flex", "flex-col", "items-center")}>
                    <IconLoading class={classes!("text-sky-400", "my-6")} />
                    {"Loading..."}
                </div>
            </div>
        }
    };
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
