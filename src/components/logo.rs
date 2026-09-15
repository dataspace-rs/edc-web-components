use patternfly_yew::prelude::Skeleton;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct LogoProps {
  #[prop_or_default]
  pub url: Option<String>,
  #[prop_or_default]
  pub disabled: bool,
  #[prop_or("250px".to_string())]
  pub height: String,
  #[prop_or_default]
  pub width: Option<String>,
}

#[component]
pub fn Logo(props: &LogoProps) -> Html {
  let style = format!(
    "height: {}; width: {}; object-fit: contain;",
    props.height,
    props.width.clone().unwrap_or("auto".to_string())
  );

  match &props.url {
    Some(url) => html! { <img src={url.to_string()} {style} /> },
    None => html! { <Skeleton {style} /> },
  }
}
