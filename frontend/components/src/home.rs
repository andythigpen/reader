use yew::prelude::*;

use crate::article_list::ArticleList;
use crate::footer::Footer;
use crate::header::Header;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class={classes!("flex", "flex-col", "items-center", "dark:bg-slate-900", "min-h-screen")}>
            <Header/>
            <ArticleList/>
            <Footer/>
        </div>
    }
}
