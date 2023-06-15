use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(IconXMark)]
pub fn icon_x_mark(props: &Props) -> Html {
    let Props { class } = props;
    html! {
        <svg xmlns={"http://www.w3.org/2000/svg"} fill={"none"} viewBox={"0 0 24 24"} stroke-width={"1.5"} stroke={"currentColor"} class={classes!("w-6", "h-6", class.clone())}>
            <path stroke-linecap={"round"} stroke-linejoin={"round"} d={"M6 18L18 6M6 6l12 12"} />
        </svg>
    }
}
