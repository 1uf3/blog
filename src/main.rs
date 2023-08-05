mod components;
mod layouts;
mod pages;
mod route;

use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

use crate::route::{switch, Route};

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
