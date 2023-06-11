use entity::article::Model;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(ArticleList)]
pub fn article_list() -> Html {
    let data = use_state(|| None);

    {
        let data = data.clone();
        use_effect(move || {
            if data.is_none() {
                spawn_local(async move {
                    let resp = Request::get("/api/articles").send().await.unwrap();
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
                    data.set(Some(result));
                })
            }

            || {}
        });
    }

    let articles = match data.as_ref() {
        None => html! { <div>{"Loading..."}</div> },
        Some(Ok(data)) => {
            html! { <div>{for data.iter().map(|it|  it.title.to_owned() )}</div> }
        }
        Some(Err(err)) => html! { <div>{err}</div> },
    };

    html! {
        <div>
            {articles}
            // <Article />
        </div>
    }
}
