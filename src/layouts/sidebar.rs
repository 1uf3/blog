use yew::{function_component, html, Html};

use crate::components::{
    navigator::Navigator,
    profile::{Profile, Profiler},
    social_list::{SocialProps, Socials},
};

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    let profile = Profile {
        name: "lufe".to_string(),
        subtitle: "lufe site".to_string(),
    };
    let socials = vec![
        SocialProps {
            url: url::Url::parse("https://github.com/1uf3").unwrap(),
        },
        SocialProps {
            url: url::Url::parse("https://twitter.com/lufe_t").unwrap(),
        },
    ];
    html! {
    <div id="sidebar" class="d-flex flex-column align-items-end">
        <Profiler profile={profile} />
        <Navigator />
        <Socials socials={socials} />
    </div>
    }
}
