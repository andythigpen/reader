use gloo_net::http::Request;
use stores::filter::FilterStore;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::button::Button;
use crate::filter::Filter;
use crate::filter_form::{FilterForm, ModalAction};
use crate::icons::plus::IconPlus;
use crate::list_item::ListItem;
use crate::modal::Modal;

#[function_component(FilterList)]
pub fn filter_list() -> Html {
    let filters = use_selector(|s: &FilterStore| s.filters.clone());
    let display_modal = use_state(|| false);
    let edit_model = use_state(|| None);

    {
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    Dispatch::<FilterStore>::new()
                        .reduce_mut_future(|s| Box::pin(async move { s.fetch().await }))
                        .await;
                });
                || ()
            },
            (),
        );
    }

    let filters = if filters.is_empty() {
        html! {
            <ListItem>{"No filters. Click the Add button above to add one."}</ListItem>
        }
    } else {
        filters
            .iter()
            .enumerate()
            .map(|(id, _)| {
                html! { <Filter key={id} {id} /> }
            })
            .collect::<Html>()
    };

    let add_filter = {
        let display_modal = display_modal.clone();
        Callback::from(move |_| display_modal.set(true))
    };

    let close_modal = {
        let display_modal = display_modal.clone();
        Callback::from(move |_| display_modal.set(false))
    };

    let close_form = {
        let display_modal = display_modal.clone();
        Callback::from(move |action| match action {
            ModalAction::Close => display_modal.set(false),
            ModalAction::Confirm(model) => {
                display_modal.set(false);
                spawn_local(async move {
                    let resp = Request::post("/api/filters")
                        .json(&model)
                        .unwrap()
                        .send()
                        .await
                        .unwrap();
                    let model = resp.json().await.unwrap();
                    Dispatch::<FilterStore>::new().reduce_mut(|s| s.filters.insert(0, model));
                });
            }
        })
    };

    html! {
        <>
            <Modal display={*display_modal} onclose={close_modal}>
                <FilterForm model={(*edit_model).to_owned()} onclose={close_form} />
            </Modal>
            <div class={classes!("flex", "flex-row", "justify-between", "items-center")}>
                <h2 class={classes!("flex-1", "text-xl")}>{"Filters"}</h2>
                <Button onclick={add_filter} primary=true>
                    <IconPlus />
                    {"Add"}
                </Button>
            </div>
            { filters }
        </>
    }
}
