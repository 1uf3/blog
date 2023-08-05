use yew::{function_component, html, Html};

#[function_component(LatestContentModification)]
pub fn latest_content_modification() -> Html {
    html! {
        <div id="content-lastmod" class="post">
            <div class="panel-heading">{"Recently Updated"}</div>
            <ul class="post-content list-unstyled ps-0 pb-1 ms-1 mt-2">
                <li class="text-truncate lh-lg">
                    <a href="/posts/getting-started/">{"Getting Started"}</a>
                </li>
                <li class="text-truncate lh-lg">
                    <a href="/posts/text-and-typography/">{"Text and Typography"}</a>
                </li>
            </ul>
        </div>
    }
}
