use yew::{function_component, html, Html};

use crate::route::Route;
use strum::IntoEnumIterator;

#[function_component(Navigator)]
pub fn navigator() -> Html {
    let routes = Route::iter()
        .filter(|v| {
            let ex = vec![Route::NotFound, Route::Post { id: "".into() }];
            !ex.contains(v)
        })
        .collect::<Vec<_>>();
    let routes = routes
        .iter()
        .map(|r| link_list(r.to_string()))
        .collect::<Html>();
    html! {
        <ul class="nav flex-column flex-grow-1 w-100 ps-0">
            {routes}
        </ul>
    }
}

fn link_list(s: String) -> Html {
    html! {
        <li class="nav-item">
            <a href={format!("/{}/", s.to_lowercase())} class="nav-link">
                <i class="fa-fw fas fa-info-circle"></i>
                <span>{s.to_uppercase()}</span>
            </a>
        </li>
    }
}
