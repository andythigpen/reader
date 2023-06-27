use yew::prelude::*;

use crate::icons::x_mark::IconXMark;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub display: bool,

    #[prop_or_default]
    pub onclose: Callback<MouseEvent>,
}

#[function_component(Modal)]
pub fn modal(
    Props {
        children,
        display,
        onclose,
    }: &Props,
) -> Html {
    if !display {
        return html! {};
    }

    let onclose = onclose.clone();
    let onclick = Callback::from(move |e| onclose.emit(e));

    html! {
        <div class={classes!(
            "fixed", "py-12", "bg-gray-800/75", "transition", "duration-150", "ease-in-out",
            "z-50", "top-0", "right-0", "bottom-0", "left-0", "dark:text-white"
        )}>
            <div class={classes!("container", "mx-auto", "w-11/12", "md:w-2/3", "max-w-lg", "z-10")}>
                <div class={classes!(
                    "relative", "py-8", "px-5", "md:px-10", "dark:bg-slate-700", "shadow-md",
                    "rounded-lg", "border", "dark:border-slate-800", "h-fit", "max-h-[calc(100vh-6rem)]"
                )}>
                    { for children.iter() }
                    <button class={classes!(
                        "cursor-pointer", "absolute", "top-0", "right-0", "mt-4", "mr-5",
                        "text-gray-400", "hover:text-gray-600", "transition", "duration-150",
                        "ease-in-out", "rounded", "focus:ring-2", "focus:outline-none", "focus:ring-gray-600"
                    )} aria-label={"close modal"} role={"button"} {onclick}>
                        <IconXMark/>
                    </button>
                </div>
            </div>
        </div>
    }
}
