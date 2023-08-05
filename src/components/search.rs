use yew::{function_component, html, Html};

#[function_component(SearchResult)]
pub fn search_result() -> Html {
    html! {
        <div id="search-result-wrapper" class="d-flex justify-content-center unloaded">
            <div class="col-11 post-content">
                <div id="search-results" class="d-flex flex-wrap justify-content-center text-muted mt-3">
                </div>
            </div>
        </div>
    }
}