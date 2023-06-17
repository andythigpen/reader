use gloo_net::http::Request;
use stores::category::CategoryStore;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::Properties;
use yewdux::prelude::*;

use crate::button::Button;
use crate::category_form::CategoryForm;
use crate::category_form::ModalAction;
use crate::icons::pencil_square::IconPencilSquare;
use crate::icons::trash::IconTrash;
use crate::list_item::ListItem;
use crate::modal::Modal;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub id: usize,
}

#[function_component(Category)]
pub fn category(&Props { id }: &Props) -> Html {
    let display_edit_modal = use_state(|| false);
    let display_delete_modal = use_state(|| false);
    let store = use_store_value::<CategoryStore>();
    let model = &store.categories[id];

    let edit = {
        let display_edit_modal = display_edit_modal.clone();
        Callback::from(move |_| display_edit_modal.set(true))
    };

    let close_edit_modal = {
        let display_edit_modal = display_edit_modal.clone();
        Callback::from(move |_| display_edit_modal.set(false))
    };

    let close_form = {
        let display_edit_modal = display_edit_modal.clone();
        Callback::from(move |action| match action {
            ModalAction::Close => display_edit_modal.set(false),
            ModalAction::Confirm(model) => {
                let display_edit_modal = display_edit_modal.clone();
                spawn_local(async move {
                    Request::put(&format!("/api/categories/{}", model.id))
                        .json(&model)
                        .unwrap()
                        .send()
                        .await
                        .unwrap();
                    Dispatch::<CategoryStore>::new().reduce_mut(|s| s.categories[id] = model);
                    display_edit_modal.set(false);
                });
            }
        })
    };

    let delete = {
        let display_delete_modal = display_delete_modal.clone();
        Callback::from(move |_| display_delete_modal.set(true))
    };

    let close_delete_modal = {
        let display_delete_modal = display_delete_modal.clone();
        Callback::from(move |_| display_delete_modal.set(false))
    };

    let delete_confirm = {
        let model_id = model.id.clone();
        let display_delete_modal = display_delete_modal.clone();
        Callback::from(move |_| {
            let model_id = model_id.clone();
            let display_delete_modal = display_delete_modal.clone();
            spawn_local(async move {
                Request::delete(&format!("/api/categories/{}", model_id))
                    .send()
                    .await
                    .unwrap();
                Dispatch::<CategoryStore>::new().reduce_mut(|s| s.categories.remove(id));
                display_delete_modal.set(false);
            });
        })
    };

    html! {
        <ListItem>
            <Modal display={*display_edit_modal} onclose={close_edit_modal}>
                <CategoryForm model={model.to_owned()} onclose={close_form} />
            </Modal>
            <Modal display={*display_delete_modal} onclose={close_delete_modal.clone()}>
                <h1 class={classes!("text-xl", "mb-4")}>{"Delete Category"}</h1>

                <p class={classes!("m-4")}>{model.name.clone()}</p>

                <div class={classes!("flex", "flex-row", "justify-end", "gap-1")}>
                    <Button onclick={close_delete_modal}>{"Cancel"}</Button>
                    <Button onclick={delete_confirm} primary=true>{"Delete"}</Button>
                </div>
            </Modal>
            <div class={classes!("flex", "flex-col", "flex-1")}>
                <h2 class={classes!("dark:text-white", "text-lg")}>{model.name.clone()}</h2>
                <p>{model.description.clone()}</p>
            </div>
            <div class={classes!("flex", "flex-row", "items-center", "gap-4")}>
                <a onclick={edit}>
                    <IconPencilSquare class={classes!("cursor-pointer")} />
                </a>
                <a onclick={delete}>
                    <IconTrash class={classes!("cursor-pointer")} />
                </a>
            </div>
        </ListItem>
    }
}
