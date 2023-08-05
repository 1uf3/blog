use yew::Html;
use yew_router::prelude::*;

use crate::pages::{
    about::about, archives::archives, categories::categories, home::home, tags::tags,
    _404::not_found,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/categories")]
    Categories,
    #[at("/tags")]
    Tags,
    #[at("/archives")]
    Archives,
    #[at("/about")]
    About,
    #[at("/post/:id")]
    Post { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => home(),
        Route::Categories => categories(),
        Route::Tags => tags(),
        Route::Archives => archives(),
        Route::About => about(),
        _ => not_found(),
    }
}
