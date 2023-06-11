use yew::prelude::*;

use crate::article_list::ArticleList;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <h1>{"Reader"}</h1>
            <ArticleList/>
        </>
    }
}
