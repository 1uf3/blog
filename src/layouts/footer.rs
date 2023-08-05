use chrono::Datelike;
use yew::{function_component, html, Html};

#[function_component(Footer)]
pub fn footer() -> Html {
    let current_date = chrono::Utc::now();
    html! {
        <footer>
            <div class="container px-lg-4">
                <div class="d-flex justify-content-center align-items-center text-muted mx-md-3">
                    <p>
                        {format!("CopyRight © {} ", current_date.year())}
                        <a href="https://github.com/1uf3">{"lufe"}</a>
                        {". "}
                        <span data-bs-toggle="tooltip" data-bs-placement="top" data-bs-original-title="Except where otherwise noted, the blog posts on this site are licensed under the Creative Commons Attribution 4.0 International (CC BY 4.0) License by the author.">{"Some rights reserved."}</span>
                    </p>
                </div>
            </div>
        </footer>
    }
}
