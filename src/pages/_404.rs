use yew::{html, Html};

use crate::{
    components::major::Major,
    layouts::{footer::Footer, sidebar::Sidebar},
};

pub fn not_found() -> Html {
    html! {
        <>
            <Major title={"404"}>
                <h1>{"404 Not Found"}</h1>
            </Major>
            <Sidebar></Sidebar>
            <Footer></Footer>
        </>
    }
}
