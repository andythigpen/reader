// use entity::article::Model;
use stores::article::ArticleStore;
use yew::prelude::*;
use yew::Properties;
use yewdux::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    // pub article: Model,
    pub index: usize,
}

#[function_component(Article)]
pub fn article(props: &Props) -> Html {
    let (store, _) = use_store::<ArticleStore>();
    let article = &store.articles[props.index];
    html! {
        <article>
            <h2>{article.title.clone()}</h2>
            <p>{article.description.clone()}</p>
        </article>
    }
}
