use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;

/// Scroll the page to the top.
///
/// Because the DOM may still be updating when this is called, the scroll is
/// performed immediately and then scheduled again after yielding to the event
/// loop. This makes it reliable on mobile Chrome where synchronous
/// `window.scrollTo` calls are sometimes ignored.
pub fn scroll_to_top() {
    scroll_to_top_now();
    spawn_local(async move {
        TimeoutFuture::new(0).await;
        scroll_to_top_now();
    });
}

fn scroll_to_top_now() {
    if let Some(window) = window() {
        window.scroll_with_x_and_y(0.0, 0.0);
        if let Some(document) = window.document() {
            if let Some(html) = document.document_element() {
                html.set_scroll_top(0);
            }
            if let Some(body) = document.body() {
                body.set_scroll_top(0);
            }
        }
    }
}
