use yew::prelude::*;  
use yew_router::prelude::*;
use chrono::Datelike;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/categories")]
    CATEGORIES,
    #[at("/tags")]
    TAGS,
    #[at("/archives")]
    ARCHIVES,
    #[at("/about")]
    ABOUT,
    #[at("/post/:id")]
    Post { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => home(),
        Route::CATEGORIES => categories(),
        Route::TAGS => tags(),
        Route::ARCHIVES => archives(),
        Route::ABOUT => about(),
        _ => html! {
            <h1>{"404"}</h1>
        },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

fn home() -> Html {
    html! {
        <>
            <Major>
                <PostList>
                </PostList>
            </Major>
            <Sidebar></Sidebar>
            <Footer></Footer>
        </>
    }
}

fn categories() -> Html {
    html! {
        <>
            <Major title={Categories}>
                <PostList>
                </PostList>
            </Major>
            <Sidebar></Sidebar>
            <Footer></Footer>
        </>
    }
}

fn tags() -> Html {
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

fn archives() -> Html {
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

fn about() -> Html {
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

#[derive(Properties, PartialEq, Clone)]
pub struct MajorProps {
    pub children: Children,
    title: String,
}

#[function_component(Major)]
fn major(props: &MajorProps) -> Html {
    let MajorProps { children, title } = props.clone();
    html! {
        <div id="main-wrapper" class="d-flex justify-content-center">
            <div id="main" class="container px-xxl-5">
                <Toolbar title={title}></Toolbar>
                <Core>
                    {children}
                </Core>
                <Panel></Panel>
                <SearchResult></SearchResult>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct CoreProps {
    pub children: Children,
}

#[function_component(Core)]
fn core(props: &CoreProps) -> Html{
    let CoreProps { children } = props.clone();
    html! {
        <div id="core-wrapper" class="row mb-5">
            <div id="core" class="col-12 col-lg-11 col-xl-9 pe-xl-4">
                <div class="post px-md-2">
                    {children}
                </div>
            </div>
        </div>
    }
}

#[function_component(Panel)]
fn panel() -> Html{
    html!{
        <div id="panel-wrapper" class="col-xl-3 ps-2 text-muted">
            <div id="panel">
                <LatestContentModification></LatestContentModification>
                <TrendTag></TrendTag>
            </div>
        </div>
    }
}

#[function_component(LatestContentModification)]
fn latest_content_modification() -> Html{
    html!{
        <div id="content-lastmod" class="post">
            <div class="panel-heading">{"Recently Updated"}</div>
            <ul class="post-content list-unstyled ps-0 pb-1 ms-1 mt-2">
                <li class="text-truncate lh-lg"> 
                    <a href="/posts/getting-started/">{"Getting Started"}</a>
                </li>
                <li class="text-truncate lh-lg"> 
                    <a href="/posts/text-and-typography/">{"Text and Typography"}</a>
                </li>
            </ul>
        </div>
    }
}

#[function_component(SearchResult)]
fn search_result() -> Html{
    html! {
        <div id="search-result-wrapper" class="d-flex justify-content-center unloaded">
            <div class="col-11 post-content">
                <div id="search-results" class="d-flex flex-wrap justify-content-center text-muted mt-3">
                </div>
            </div>
        </div>
    }
}

#[function_component(TrendTag)]
fn trend_tag() -> Html{
    html!{
        <div id="trend-tags">
            <div class="panel-heading">{"Trending Tags"}</div>
            <div class="d-flex flex-wrap mt-3 mb-1 me-3"> 
                <a class="post-tag btn btn-outline-primary" href="/tags/favicon/">{"favicon"}</a> 
                <a class="post-tag btn btn-outline-primary" href="/tags/getting-started/">{"getting started"}</a> 
                <a class="post-tag btn btn-outline-primary" href="/tags/typography/">{"typography"}</a> 
                <a class="post-tag btn btn-outline-primary" href="/tags/writing/">{"writing"}</a>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ToolbarProps {
    title: String,
}

#[function_component(Toolbar)]
fn toolbar(props: &ToolbarProps) -> Html{
    let ToolbarProps { title } = props.clone();
    html! {
        <div id="topbar-wrapper">
            <div id="topbar" class="container d-flex align-items-center justify-content-between h-100"> 
                <span id="breadcrumb"> 
                    <span>{"Home"}</span> 
                </span> 
                <i id="sidebar-trigger" class="fas fa-bars fa-fw"></i>
                <div id="topbar-title" class=""> {title} </div>
                <i id="search-trigger" class="fas fa-search fa-fw"></i> 
                <span id="search-wrapper" class="align-items-center"> 
                    <i class="fas fa-search fa-fw"></i> 
                    <input class="form-control" id="search-input" type="search" aria-label="search" autocomplete="off" placeholder="Search..." /> 
                </span> 
                <span id="search-cancel" class="">{"Cancel"}</span>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct PostListProps {
    posts: Vec<Post>
}

#[derive(Properties, PartialEq, Clone)]
struct Post {
    title: String,
    description: String,
    image_url: Option<url::Url>,
    date: String,
    categories: Vec<String>
}

#[function_component(PostList)]
fn post_list() -> Html{
    let posts = vec![
        Post {
            title: "Getting Started".to_string(),
            description: "Prerequisites Follow the instructions in the Jekyll Docs to complete the installation of the basic environment. Git also needs to be installed. Installation Creating a New Site There are two wa...".to_string(),
            image_url: None,
            date: "Aug 9, 2019".to_string(),
            categories: vec![
                "Blogging".to_string(),
                "Tutorial".to_string(),
            ],
        },
        Post {
            title: "Text and Typography".to_string(),
            description: r"This post is to show Markdown syntax rendering on Chirpy, you can also use it as an example of writing. Now, let\’s start looking at text and typography. Headings H1 - heading H2 - heading H3 - ...".to_string(),
            image_url: Some(url::Url::parse("https://chirpy-img.netlify.app/commons/devices-mockup.png").unwrap()),
            date: "Aug 8, 2019".to_string(),
            categories: vec![
                "Blogging".to_string(),
                "Demo".to_string(),
            ],
        },
    ];
    let posts = posts.iter().map(|post| article(post)).collect::<Html>();
    html! {
        <div id="post-list">{posts}</div>
    }
}

fn article(Post{title, description, image_url, date, categories}: &Post) -> Html{
    html! {
        <a href={format!{"/posts/{}/", title.replace(" ", "-").to_ascii_lowercase()}} class="card-wrapper">
            <div class="card post-preview flex-md-row-reverse">
                <PreviewImage image_url={image_url.clone()} />
                <div class="card-body d-flex flex-column">
                    <h1 class="card-title my-2 mt-md-0"> {title}</h1>
                    <div class="card-text post-content mt-0 mb-2">
                        <p> {description}</p>
                    </div>
                    <div class="post-meta flex-grow-1 d-flex align-items-end">
                        <div class="me-auto"> 
                            <i class="far fa-calendar fa-fw me-1"></i> 
                            <em class="">{date}</em> 
                            <i class="far fa-folder-open fa-fw me-1"></i> 
                            <span class="categories"> {categories.join(", ")} </span>
                        </div>
                        <div class="pin ms-1"> 
                            <i class="fas fa-thumbtack fa-fw"></i> 
                            <span></span>
                        </div>
                    </div>
                </div>
            </div>
        </a>
    }
}

#[derive(Properties, PartialEq)]
struct PreviewImageProps{
    image_url: Option<url::Url>
}

#[function_component(PreviewImage)]
fn preview_image(PreviewImageProps{image_url}: &PreviewImageProps) -> Html {
    if let Some(image_url) = image_url {
        html! {
            <div class="preview-img">
                <img data-src={image_url.to_string()} width="17" height="10" alt="responsive rendering of chirpy theme on multiple devices." data-lqip="true" src={image_url.to_string()} class=" lazyloaded" data-proofer-ignore="" />
            </div>
        }
    } else {
        html! {
            <div class="preview-img"></div>
        }
    }
}

#[function_component(Sidebar)]
fn sidebar() -> Html {
    let profile = Profile { name: "lufe".to_string(), subtitle: "lufe site".to_string() };
    let links = vec![
        LinkProps{
            s: "home".to_string()
        },
        LinkProps{
            s: "categories".to_string()
        },
        LinkProps{
            s: "tags".to_string()
        },
        LinkProps{
            s: "archives".to_string()
        },
        LinkProps{
            s: "about".to_string()
        },
    ];
    let socials= vec![
        SocialProps {
            url: url::Url::parse("https://github.com/1uf3").unwrap()
        },
        SocialProps {
            url: url::Url::parse("https://twitter.com/lufe_t").unwrap()
        }
    ];
    html! {
    <>
        <div id="sidebar" class="d-flex flex-column align-items-end">
            <Profiler profile={profile} />
            <Navigator links={links} />
            <Socials socials={socials} />
        </div>
    </>
    }
}

#[derive(Properties, PartialEq)]
struct ProfileProps{
    profile: Profile
}

#[derive(Properties, PartialEq, Clone)]
struct Profile{
    name: String,
    subtitle: String
}

#[function_component(Profiler)]
fn profile(ProfileProps{profile}: &ProfileProps) -> Html {
    let profile = profile.clone();
    html!{
        <div class="profile-wrapper">
            <a href="/" id="avatar" class="rounded-circle"></a>
            <div class="site-title">
                <a href="/">{profile.name}</a>
            </div>
            <div class="site-subtitle fst-italic">{profile.subtitle}</div>
        </div>

    }
}

#[derive(Properties, PartialEq)]
struct NavigatorProps{
    links: Vec<LinkProps>
}

#[function_component(Navigator)]
fn navigator(NavigatorProps{ links }: &NavigatorProps) -> Html {
    let links = links.iter().map(|link| link_list(link)).collect::<Html>();
    html!{
        <ul class="nav flex-column flex-grow-1 w-100 ps-0">
            {links}
        </ul>
    }
}

#[derive(Properties, PartialEq)]
struct LinkProps {
    s: String
}

fn link_list(LinkProps{ s }: &LinkProps) -> Html {
    html!{
        <li class="nav-item"> 
            <a href={format!("/{}/", s)} class="nav-link"> 
                <i class="fa-fw fas fa-info-circle"></i> 
                <span>{s.to_uppercase()}</span>
            </a>
        </li>
    }
}

#[derive(Properties, PartialEq)]
struct SocialsProps{
    socials: Vec<SocialProps>
}

#[function_component(Socials)]
fn socials(SocialsProps{ socials}: &SocialsProps) -> Html {
    let socials = socials.iter().map(|social| social_list(social)).collect::<Html>();
    html!{
        <div class="sidebar-bottom d-flex flex-wrap align-items-center w-100"> 
            {socials}
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct SocialProps {
    url: url::Url
}

fn social_list(SocialProps{ url }: &SocialProps) -> Html {
    html!{
        <a href={url.to_string()} aria-label="twitter" target="_blank" rel="noopener noreferrer"> 
            <i class="fab fa-twitter"></i> 
        </a> 
    }
}

#[function_component(Footer)]
fn fotter() -> Html {
    let current_date = chrono::Utc::now();
    html!{
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