use dto;
use gloo_net::http::Request;
use gloo_utils::document;
use web_sys::{Element, Node};
use yew::{
    prelude::*,
    suspense::{use_future, SuspensionResult},
};

use crate::date::Date;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

#[hook]
fn use_article(id: String) -> SuspensionResult<dto::ReadabilityArticle> {
    let s = use_future(|| async move {
        let resp: dto::ReadabilityArticle =
            Request::get(&format!("/api/articles/{}/readability", id))
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();
        resp
    })?;
    Ok((*s).clone())
}

#[function_component(ReadabilityArticle)]
pub fn readability_article(props: &Props) -> HtmlResult {
    let article: dto::ReadabilityArticle = use_article(props.id.clone())?;
    let title = article.title.clone();
    let pub_date = article.pub_date.clone();
    let url = article.url.clone();

    let node = use_memo(
        |(article,)| {
            let div: Element = document().create_element("article").unwrap();
            div.set_attribute(
                "class",
                "[&>p]:my-6 [&_a]:text-sky-400 [&_code]:overflow-auto [&_code]:block text-xl leading-8 md:max-w-2xl max-w-full",
            )
            .unwrap();
            div.set_inner_html(&article.content);
            let node: Node = div.into();
            Html::VRef(node)
        },
        (article,),
    );

    Ok(html! {
        <div class={classes!("flex", "flex-col", "items-center", "m-2", "md:my-4")}>
            <h1 class={classes!("md:max-w-2xl", "text-3xl")}>{title}</h1>
            <aside class={classes!("text-sm", "my-4")}>
                {"Published at "}<Date value={pub_date}/>
                {" | "}
                <a href={url} class={classes!("text-sky-400")}>{"Original Article"}</a>
            </aside>
            {(*node).clone()}
        </div>
    })
}
