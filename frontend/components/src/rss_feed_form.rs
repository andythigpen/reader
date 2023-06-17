use entity::rss_feed::Model;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::button::Button;
use crate::input_checkbox::InputCheckbox;
use crate::input_color::InputColor;
use crate::input_text::InputText;

pub enum ModalAction {
    Close,
    Confirm(Model),
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub model: Option<Model>,

    #[prop_or_default]
    pub onclose: Callback<ModalAction>,
}

#[function_component(RssFeedForm)]
pub fn rss_feed_form(props: &Props) -> Html {
    let model = use_state(|| {
        props
            .model
            .to_owned()
            .unwrap_or(Model {
                id: "".to_string(),
                name: "".to_string(),
                description: "".to_string(),
                url: "".to_string(),
                created_at: "".to_string(),
                updated_at: "".to_string(),
                display_description: false,
                abbreviation: "".to_string(),
                color: "#6590D5".to_string(),
            })
            .clone()
    });
    let action = if props.model.is_some() {
        "Update"
    } else {
        "Add"
    };

    let cancel = {
        let onclose = props.onclose.clone();
        Callback::from(move |_| onclose.emit(ModalAction::Close))
    };
    let confirm = {
        let onclose = props.onclose.clone();
        let model = model.clone();
        Callback::from(move |_| onclose.emit(ModalAction::Confirm((*model).to_owned())))
    };
    let blur_name = {
        let model = model.clone();
        Callback::from(move |e: FocusEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            model.set(Model {
                name: input.value(),
                ..(*model).clone()
            });
        })
    };
    let blur_description = {
        let model = model.clone();
        Callback::from(move |e: FocusEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            model.set(Model {
                description: input.value(),
                ..(*model).clone()
            });
        })
    };
    let blur_abbreviation = {
        let model = model.clone();
        Callback::from(move |e: FocusEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            model.set(Model {
                abbreviation: input.value(),
                ..(*model).clone()
            });
        })
    };
    let blur_url = {
        let model = model.clone();
        Callback::from(move |e: FocusEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            model.set(Model {
                url: input.value(),
                ..(*model).clone()
            });
        })
    };
    let change_color = {
        let model = model.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            model.set(Model {
                color: input.value(),
                ..(*model).clone()
            });
        })
    };
    let change_display_description = {
        let model = model.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            model.set(Model {
                display_description: input.checked(),
                ..(*model).clone()
            });
        })
    };

    html! {
        <>
            <h1 class={classes!("text-xl", "mb-4")}>{action}{" RSS Feed"}</h1>

            <InputText name="name" label="Name" value={model.name.clone()} onblur={blur_name} />
            <InputText name="description" label="Description" value={model.description.clone()}
                onblur={blur_description} />
            <InputText name="abbreviation" label="Abbreviation" value={model.abbreviation.clone()} onblur={blur_abbreviation} />
            <InputText name="url" label="URL" value={model.url.clone()} onblur={blur_url} />
            <InputColor name="color" label="Color" value={model.color.clone()} onchange={change_color} />

            <InputCheckbox name="display_description" label="Display article descriptions"
                checked={model.display_description} onchange={change_display_description} />

            <div class={classes!("flex", "flex-row", "justify-end", "gap-1")}>
                <Button onclick={cancel}>{"Cancel"}</Button>
                <Button onclick={confirm} primary=true>{action}</Button>
            </div>
        </>
    }
}
