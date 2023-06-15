use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub name: String,

    pub label: String,

    #[prop_or_default]
    pub checked: bool,

    #[prop_or_default]
    pub onchange: Callback<Event>,
}

#[function_component(InputCheckbox)]
pub fn input_checkbox(
    Props {
        name,
        label,
        checked,
        onchange,
    }: &Props,
) -> Html {
    html! {
        <div class={classes!("mb-4")}>
            <input name={name.clone()} type="checkbox" class={classes!(
                "appearance-none", "w-9", "focus:outline-none", "checked:bg-blue-300", "h-5",
                "bg-gray-300", "rounded-full", "before:inline-block", "before:rounded-full",
                "before:bg-blue-500", "before:h-4", "before:w-4", "checked:before:translate-x-full",
                "shadow-inner", "transition-all", "duration-300", "before:ml-0.5",
                "mr-4", "cursor-pointer"
            )} checked={*checked} {onchange} />
            <label for={name.clone()}>{label}</label>
        </div>
    }
}
