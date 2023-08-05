use yew::{function_component, html, Html};

use crate::components::{modification::LatestContentModification, trend_tag::TrendTag};

#[function_component(Panel)]
pub fn panel() -> Html {
    html! {
        <div id="panel-wrapper" class="col-xl-3 ps-2 text-muted">
            <div id="panel">
                <LatestContentModification></LatestContentModification>
                <TrendTag></TrendTag>
            </div>
        </div>
    }
}
