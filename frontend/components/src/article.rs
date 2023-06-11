use entity::article::Model;
use yew::prelude::*;
use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub article: Model,
}

#[function_component(Article)]
pub fn article(props: &Props) -> Html {
    html! {
        <article>
            <h2>{props.article.title.clone()}</h2>
            <p>{props.article.description.clone()}</p>
        </article>
    }
}
