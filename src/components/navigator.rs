use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct NavigatorProps {
    pub links: Vec<LinkProps>,
}

#[function_component(Navigator)]
pub fn navigator(NavigatorProps { links }: &NavigatorProps) -> Html {
    let links = links.iter().map(link_list).collect::<Html>();
    html! {
        <ul class="nav flex-column flex-grow-1 w-100 ps-0">
            {links}
        </ul>
    }
}

#[derive(Properties, PartialEq)]
pub struct LinkProps {
    pub s: String,
}

fn link_list(LinkProps { s }: &LinkProps) -> Html {
    html! {
        <li class="nav-item">
            <a href={format!("/{}/", s)} class="nav-link">
                <i class="fa-fw fas fa-info-circle"></i>
                <span>{s.to_uppercase()}</span>
            </a>
        </li>
    }
}
