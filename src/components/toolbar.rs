use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct ToolbarProps {
    pub title: String,
}

#[function_component(Toolbar)]
pub fn toolbar(props: &ToolbarProps) -> Html {
    let ToolbarProps { title } = props.clone();
    html! {
        <div id="topbar-wrapper">
            <div id="topbar" class="container d-flex align-items-center justify-content-between h-100">
                <span id="breadcrumb">
                    <span>{title.clone()}</span>
                </span>
                <i id="sidebar-trigger" class="fas fa-bars fa-fw"></i>
                <div id="topbar-title" class=""> {title} </div>
                <i id="search-trigger" class="fas fa-search fa-fw"></i>
                <span id="search-wrapper" class="align-items-center">
                    <i class="fas fa-search fa-fw"></i>
                    <input class="form-control" id="search-input" type="search" aria-label="search" autocomplete="off" placeholder="Search..." />
                </span>
                <span id="search-cancel" class="">{"Cancel"}</span>
            </div>
        </div>
    }
}
