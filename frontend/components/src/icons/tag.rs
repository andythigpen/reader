use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(IconTag)]
pub fn icon_tag(props: &Props) -> Html {
    let Props { class } = props;
    html! {
        <svg xmlns={"http://www.w3.org/2000/svg"} fill={"none"} viewBox={"0 0 24 24"} stroke-width={"1.5"} stroke={"currentColor"} class={classes!("w-6", "h-6", class.clone())}>
            <path stroke-linecap={"round"} stroke-linejoin={"round"} d={"M9.568 3H5.25A2.25 2.25 0 003 5.25v4.318c0 .597.237 1.17.659 1.591l9.581 9.581c.699.699 1.78.872 2.607.33a18.095 18.095 0 005.223-5.223c.542-.827.369-1.908-.33-2.607L11.16 3.66A2.25 2.25 0 009.568 3z"} />
        </svg>
    }
}
