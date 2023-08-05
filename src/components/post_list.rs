use yew::{function_component, html, Html, Properties};

use crate::components::preview_image::PreviewImage;

#[derive(Properties, PartialEq)]
struct PostListProps {
    posts: Vec<Post>,
}

#[derive(Properties, PartialEq, Clone)]
struct Post {
    title: String,
    description: String,
    image_url: Option<url::Url>,
    date: String,
    categories: Vec<String>,
}

#[function_component(PostList)]
pub fn post_list() -> Html {
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
    let posts = posts.iter().map(article).collect::<Html>();
    html! {
        <div id="post-list">{posts}</div>
    }
}

fn article(
    Post {
        title,
        description,
        image_url,
        date,
        categories,
    }: &Post,
) -> Html {
    html! {
        <a href={format!{"/posts/{}/", title.replace(' ', "-").to_ascii_lowercase()}} class="card-wrapper">
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
