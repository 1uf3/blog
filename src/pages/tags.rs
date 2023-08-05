use yew::{html, Html};

use crate::{
    components::{major::Major, post_list::PostList},
    layouts::{footer::Footer, sidebar::Sidebar},
};

pub fn tags() -> Html {
    html! {
        <>
            <Major title={"Tags"}>
                <PostList>
                </PostList>
            </Major>
            <Sidebar></Sidebar>
            <Footer></Footer>
        </>
    }
}
