use yew::{function_component, html, Children, Html, Properties};

use crate::{
    components::{search::SearchResult, toolbar::Toolbar},
    layouts::panel::Panel,
};

#[derive(Properties, PartialEq, Clone)]
pub struct MajorProps {
    pub children: Children,
    pub title: String,
}

#[function_component(Major)]
pub fn major(props: &MajorProps) -> Html {
    let MajorProps { children, title } = props.clone();
    html! {
        <div id="main-wrapper" class="d-flex justify-content-center">
            <div id="main" class="container px-xxl-5">
                <Toolbar title={title}></Toolbar>
                <Core>
                    {children}
                </Core>
                <Panel></Panel>
                <SearchResult></SearchResult>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct CoreProps {
    pub children: Children,
}

#[function_component(Core)]
fn core(props: &CoreProps) -> Html {
    let CoreProps { children } = props.clone();
    html! {
        <div id="core-wrapper" class="row mb-5">
            <div id="core" class="col-12 col-lg-11 col-xl-9 pe-xl-4">
                <div class="post px-md-2">
                    {children}
                </div>
            </div>
        </div>
    }
}
