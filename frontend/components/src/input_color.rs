use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub name: String,

    pub label: String,

    #[prop_or_default]
    pub value: String,

    #[prop_or_default]
    pub onchange: Callback<Event>,
}

#[function_component(InputColor)]
pub fn input_color(
    Props {
        name,
        label,
        value,
        onchange,
    }: &Props,
) -> Html {
    html! {
        <>
            <label for={name.clone()}>{label}</label>
            <input name={name.clone()} class={classes!(
                "mb-5", "mt-2", "text-gray-600", "focus:outline-none", "focus:border",
                "focus:border-indigo-700", "font-normal", "w-full", "h-10", "flex",
                "items-center", "pl-3", "text-sm", "border-gray-300", "rounded", "border"
            )} type="color" placeholder={label.clone()} value={value.clone()} {onchange} />
        </>
    }
}
