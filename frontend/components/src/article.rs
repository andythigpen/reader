use stores::article::ArticleStore;
use time::{format_description, format_description::well_known::Iso8601, OffsetDateTime};
use yew::prelude::*;
use yew::Properties;
use yewdux::prelude::*;

use crate::icons::arrow_top_right::IconArrowTopRight;
use crate::icons::chat_bubble_left_ellipsis::IconChatBubbleLeftEllipsis;
use crate::list_item::ListItem;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub id: usize,
}

#[function_component(Article)]
pub fn article(&Props { id }: &Props) -> Html {
    let store = use_store_value::<ArticleStore>();
    let article = &store.articles[id];

    let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]").unwrap();
    let pub_date = OffsetDateTime::parse(&article.pub_date, &Iso8601::DEFAULT)
        .unwrap()
        .format(&format)
        .unwrap();

    html! {
        <ListItem>
            <a href={article.url.clone()} class={classes!("flex-1")}>
                <h2 class={classes!("text-lg", "dark:text-white")}>
                    {article.title.clone()}
                </h2>
                <span class={classes!("text-sm")}>{"Published at "}{pub_date}</span>
                <p>{article.description.clone()}</p>
            </a>
            if let Some(comments_url) = &article.comments_url {
                <a href={comments_url.clone()} class={classes!("flex-0", "mr-4")} target={"_blank"} rel={"noopener noreferrer"}>
                    <IconChatBubbleLeftEllipsis/>
                </a>
            }
            <a href={article.url.clone()} class={classes!("flex-0")} target={"_blank"} rel={"noopener noreferrer"}>
                <IconArrowTopRight/>
            </a>
        </ListItem>
    }
}
