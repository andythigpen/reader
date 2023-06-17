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
            "md:p-6", "mr-3", "md:mr-4", "w-16", "md:w-20",
            "text-white", "text-2xl", "rounded-lg", "text-center",
            "flex", "flex-col", "justify-center", "items-center"
        )}>
            {text}
        </div>
    }
}
