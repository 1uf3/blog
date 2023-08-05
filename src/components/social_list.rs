use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct SocialsProps {
    pub socials: Vec<SocialProps>,
}

#[function_component(Socials)]
pub fn socials(SocialsProps { socials }: &SocialsProps) -> Html {
    let socials = socials.iter().map(social_list).collect::<Html>();
    html! {
        <div class="sidebar-bottom d-flex flex-wrap align-items-center w-100">
            {socials}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct SocialProps {
    pub url: url::Url,
}

pub fn social_list(SocialProps { url }: &SocialProps) -> Html {
    html! {
        <a href={url.to_string()} aria-label="twitter" target="_blank" rel="noopener noreferrer">
            <i class="fab fa-twitter"></i>
        </a>
    }
}
