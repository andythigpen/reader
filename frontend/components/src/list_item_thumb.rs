use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or_default]
    pub text: String,

    #[prop_or_default]
    pub color: String,
}

#[function_component(ListItemThumb)]
pub fn list_item_thumb(Props { text, color }: &Props) -> Html {
    html! {
        <div style={format!("background-color: {};", color)} class={classes!(
            "p-6", "text-white", "text-2xl", "rounded-lg", "mr-4", "w-20", "text-center"
        )}>
            {text}
        </div>
    }
}
