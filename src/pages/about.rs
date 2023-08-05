use yew::{html, Html};

use crate::{
    components::{major::Major, post_list::PostList},
    layouts::{footer::Footer, sidebar::Sidebar},
};

pub fn about() -> Html {
    html! {
        <>
            <Major title={"About"}>
                <PostList>
                </PostList>
            </Major>
            <Sidebar></Sidebar>
            <Footer></Footer>
        </>
    }
}
