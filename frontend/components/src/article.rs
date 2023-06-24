use router::Route;
use stores::article::ArticleStore;
use stores::rss_feed::RssFeedStore;
use yew::prelude::*;
use yew::Properties;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::date::Date;
use crate::icons::{
    arrow_top_right::IconArrowTopRight, chat_bubble_left_ellipsis::IconChatBubbleLeftEllipsis,
};
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

    html! {
        <ListItem>
            <Link<Route> to={Route::RssFeedArticles{ id: article.rss_feed_id.clone() }}>
                <ListItemThumb text={abbreviation} color={thumb_color} />
            </Link<Route>>
            <div class={classes!("flex", "flex-col", "md:flex-row", "flex-1", "md:items-center", "justify-between")}>
                <Link<Route> to={Route::ReadabilityArticle{ id: article.id.clone() }}>
                    <h2 class={classes!("text-lg", "dark:text-white")}>
                        {article.title.clone()}
                    </h2>
                    <span class={classes!("text-sm")}>{"Published on "}{rss_feed_name}{" at "}<Date value={article.pub_date.clone()}/></span>
                    <p>{article.description.clone()}</p>
                </Link<Route>>
                <div class={classes!("flex", "flex-row", "flex-0", "mr-4", "mt-2", "gap-4")} >
                    if let Some(comments_url) = &article.comments_url {
                        <a href={comments_url.clone()} target={"_blank"} rel={"noopener noreferrer"} class={classes!(
                            "flex", "flex-row", "gap-1"
                        )}>
                            <IconChatBubbleLeftEllipsis/>
                            <span class={"md:hidden"}>{"Comments"}</span>
                        </a>
                    }
                    <a href={article.url.clone()} target={"_blank"} rel={"noopener noreferrer"} class={classes!(
                        "flex", "flex-row", "gap-1"
                    )}>
                        <IconArrowTopRight />
                        <span class={"md:hidden"}>{"Article"}</span>
                    </a>
                </div>
            </div>
        </ListItem>
    }
}
