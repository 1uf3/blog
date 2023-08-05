use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct PreviewImageProps {
    pub image_url: Option<url::Url>,
}

#[function_component(PreviewImage)]
pub fn preview_image(PreviewImageProps { image_url }: &PreviewImageProps) -> Html {
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
