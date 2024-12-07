use dto::Filter;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::button::Button;
use crate::input_text::InputText;

pub enum ModalAction {
    Close,
    Confirm(Filter),
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub model: Option<Filter>,

    #[prop_or_default]
    pub onclose: Callback<ModalAction>,
}

#[function_component(FilterForm)]
pub fn filter_form(props: &Props) -> Html {
    let model = use_state(|| {
        props
            .model
            .to_owned()
            .unwrap_or(Filter {
                id: "".to_string(),
                pattern: "".to_string(),
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
    let blur_pattern = {
        let model = model.clone();
        Callback::from(move |e: FocusEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            model.set(Filter {
                pattern: input.value(),
                ..(*model).clone()
            });
        })
    };

    html! {
        <>
            <h1 class={classes!("text-xl", "mb-4")}>{action}{" Filter"}</h1>

            <InputText name="pattern" label="Pattern" value={model.pattern.clone()} onblur={blur_pattern} />

            <div class={classes!("flex", "flex-row", "justify-end", "gap-1")}>
                <Button onclick={cancel}>{"Cancel"}</Button>
                <Button onclick={confirm} primary=true>{action}</Button>
            </div>
        </>
    }
}
