use wasm_bindgen::prelude::*;
use web_sys::{window, NavigationType, PerformanceNavigationTiming};
use yew::prelude::*;

#[hook]
pub fn use_page_reload<F>(f: F)
where
    F: Fn() + 'static,
{
    use_effect_with_deps(
        {
            move |_| {
                let arr = window()
                    .unwrap()
                    .performance()
                    .unwrap()
                    .get_entries_by_type("navigation");
                for elem in arr {
                    let entry: PerformanceNavigationTiming = elem.unchecked_into();
                    if entry.type_() == NavigationType::Reload {
                        f();
                    }
                }
                || ()
            }
        },
        (),
    )
}
