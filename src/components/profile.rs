use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ProfileProps {
    pub profile: Profile,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Profile {
    pub name: String,
    pub subtitle: String,
}

#[function_component(Profiler)]
pub fn profile(ProfileProps { profile }: &ProfileProps) -> Html {
    let profile = profile.clone();
    html! {
        <div class="profile-wrapper">
            <a href="/" id="avatar" class="rounded-circle"></a>
            <div class="site-title">
                <a href="/">{profile.name}</a>
            </div>
            <div class="site-subtitle fst-italic">{profile.subtitle}</div>
        </div>
    }
}
