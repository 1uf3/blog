use yew::{html, Html};

use crate::{
    components::{major::Major, post_list::PostList},
    layouts::{footer::Footer, sidebar::Sidebar},
};

pub fn archives() -> Html {
    html! {
        <>
            <Major title={"Archives"}>
                <PostList>
                </PostList>
            </Major>
            <Sidebar></Sidebar>
            <Footer></Footer>
        </>
    }
}
