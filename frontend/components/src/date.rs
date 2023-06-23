use time::{
    format_description::{self, well_known::Iso8601},
    OffsetDateTime,
};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub value: String,
}

#[function_component(Date)]
pub fn date(props: &Props) -> Html {
    let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]").unwrap();
    let date = OffsetDateTime::parse(&props.value, &Iso8601::DEFAULT)
        .unwrap()
        .format(&format)
        .unwrap();
    html! {
        <span>
            {date}
        </span>
    }
}
