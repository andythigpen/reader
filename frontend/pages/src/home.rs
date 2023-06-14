use yew::prelude::*;

use components::article_list::ArticleList;
use components::footer::Footer;
use components::header::Header;

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
