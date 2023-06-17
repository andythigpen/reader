use stores::article::ArticleStore;
use stores::rss_feed::RssFeedStore;
use time::{format_description, format_description::well_known::Iso8601, OffsetDateTime};
use yew::prelude::*;
use yew::Properties;
use yewdux::prelude::*;

use crate::icons::chat_bubble_left_ellipsis::IconChatBubbleLeftEllipsis;
use crate::list_item::ListItem;
use crate::list_item_thumb::ListItemThumb;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub id: usize,
}

#[function_component(Article)]
pub fn article(&Props { id }: &Props) -> Html {
    let article_store = use_store_value::<ArticleStore>();
    let article = &article_store.articles[id];

    let rss_feed_store = use_store_value::<RssFeedStore>();
    let rss_feed = rss_feed_store.get_by_id(&article.rss_feed_id);
    let rss_feed_name = rss_feed
        .as_ref()
        .map_or("unknown".to_string(), |r| r.name.clone());
    let abbreviation = rss_feed
        .as_ref()
        .map_or("".to_string(), |r| r.abbreviation.to_uppercase());
    let thumb_color = rss_feed.map_or("#475569".to_string(), |r| r.color.clone());

    let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]").unwrap();
    let pub_date = OffsetDateTime::parse(&article.pub_date, &Iso8601::DEFAULT)
        .unwrap()
        .format(&format)
        .unwrap();

    html! {
        <ListItem>
            <ListItemThumb text={abbreviation} color={thumb_color} />
            <div class={classes!("flex", "flex-col", "md:flex-row", "flex-1", "md:items-center", "justify-between")}>
                <a href={article.url.clone()} class={classes!("flex-1")} target={"_blank"} rel={"noopener noreferrer"}>
                    <h2 class={classes!("text-lg", "dark:text-white")}>
                        {article.title.clone()}
                    </h2>
                    <span class={classes!("text-sm")}>{"Published on "}{rss_feed_name}{" at "}{pub_date}</span>
                    <p>{article.description.clone()}</p>
                </a>
                if let Some(comments_url) = &article.comments_url {
                    <div class={classes!("flex", "flex-row", "flex-0", "mr-4", "mt-2")} >
                        <a href={comments_url.clone()} target={"_blank"} rel={"noopener noreferrer"} class={classes!(
                            "flex", "flex-row", "gap-1"
                        )}>
                            <IconChatBubbleLeftEllipsis/>
                            <span class={"md:hidden"}>{"Comments"}</span>
                        </a>
                    </div>
                }
            </div>
        </ListItem>
    }
}
