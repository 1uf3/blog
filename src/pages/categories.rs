use yew::{html, Html};

use crate::{
    components::{major::Major, post_list::PostList},
    layouts::{footer::Footer, sidebar::Sidebar},
};

pub fn categories() -> Html {
    html! {
        <>
            <Major title={"Categories"}>
                <PostList>
                </PostList>
            </Major>
            <Sidebar></Sidebar>
            <Footer></Footer>
        </>
    }
}
