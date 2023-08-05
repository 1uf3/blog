use yew::{function_component, html, Html};

#[function_component(TrendTag)]
pub fn trend_tag() -> Html {
    html! {
        <div id="trend-tags">
            <div class="panel-heading">{"Trending Tags"}</div>
            <div class="d-flex flex-wrap mt-3 mb-1 me-3">
                <a class="post-tag btn btn-outline-primary" href="/tags/favicon/">{"favicon"}</a>
                <a class="post-tag btn btn-outline-primary" href="/tags/getting-started/">{"getting started"}</a>
                <a class="post-tag btn btn-outline-primary" href="/tags/typography/">{"typography"}</a>
                <a class="post-tag btn btn-outline-primary" href="/tags/writing/">{"writing"}</a>
            </div>
        </div>
    }
}
