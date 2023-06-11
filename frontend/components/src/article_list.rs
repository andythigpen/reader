use entity::article::Model;
use gloo_net::http::Request;
use stores::article::ArticleStore;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::article::Article;

#[function_component(ArticleList)]
pub fn article_list() -> Html {
    let per_page = use_state(|| 20u64);
    let page = use_state(|| 1u64);
    let loaded = use_state(|| false);
    let (store, dispatch) = use_store::<ArticleStore>();

    {
        use_effect(move || {
            if !*loaded {
                spawn_local(async move {
                    let resp = Request::get("/api/articles")
                        .query([
                            ("page", page.to_string()),
                            ("per_page", per_page.to_string()),
                        ])
                        .send()
                        .await
                        .unwrap();
                    let result: Result<Vec<Model>, String> = {
                        if !resp.ok() {
                            Err(format!(
                                "Error fetching data {} ({})",
                                resp.status(),
                                resp.status_text()
                            ))
                        } else {
                            resp.json().await.map_err(|err| err.to_string())
                        }
                    };
                    if let Ok(mut articles) = result {
                        dispatch.reduce_mut(|s| s.articles.append(&mut articles));
                    }
                    loaded.set(true);
                });
            }
        });
    }

    html! {
        <div class={classes!("flex", "flex-col", "max-w-4xl", "container",
                             "rounded-lg", "dark:bg-slate-800")}>
            {for store.articles.iter().enumerate().map(|(idx, _)| html! { <Article index={idx} /> })}
        </div>
    }
}
