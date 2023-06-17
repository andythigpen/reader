use gloo_net::http::Request;
use stores::category::CategoryStore;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::button::Button;
use crate::category_form::{CategoryForm, ModalAction};
use crate::list_item::ListItem;
use crate::modal::Modal;
use crate::{category::Category, icons::plus::IconPlus};

#[function_component(CategoryList)]
pub fn category_list() -> Html {
    let categories = use_selector(|s: &CategoryStore| s.categories.clone());
    let display_modal = use_state(|| false);
    let edit_model = use_state(|| None);

    {
        use_effect_with_deps(
            move |_| {
                spawn_local(async move {
                    Dispatch::<CategoryStore>::new()
                        .reduce_mut_future(|s| Box::pin(async move { s.fetch().await }))
                        .await;
                });
                || ()
            },
            (),
        );
    }

    let categories = if categories.is_empty() {
        html! {
            <ListItem>{"No categories. Click the Add button above to add one."}</ListItem>
        }
    } else {
        categories
            .iter()
            .enumerate()
            .map(|(id, _)| {
                html! { <Category key={id} {id} /> }
            })
            .collect::<Html>()
    };

    let add_category = {
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
                    let resp = Request::post("/api/categories")
                        .json(&model)
                        .unwrap()
                        .send()
                        .await
                        .unwrap();
                    let model = resp.json().await.unwrap();
                    Dispatch::<CategoryStore>::new().reduce_mut(|s| s.categories.insert(0, model));
                });
            }
        })
    };

    html! {
        <>
            <Modal display={*display_modal} onclose={close_modal}>
                <CategoryForm model={(*edit_model).to_owned()} onclose={close_form} />
            </Modal>
            <div class={classes!("flex", "flex-row", "justify-between", "items-center")}>
                <h2 class={classes!("flex-1", "text-xl")}>{"Categories"}</h2>
                <Button onclick={add_category} primary=true>
                    <IconPlus />
                    {"Add"}
                </Button>
            </div>
            { categories }
        </>
    }
}
