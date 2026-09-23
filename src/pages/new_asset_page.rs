use crate::components::CreateAsset;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct NewAssetPageProps {
  #[prop_or("New Asset".to_string())]
  pub title: String,
  #[prop_or(Some("The data you hold, before any of it is offered.".to_string()))]
  pub tag_line: Option<String>,
  #[prop_or_default]
  pub company_name: Option<String>,
  #[prop_or_default]
  pub company_logo_url: Option<String>,
  pub on_create: Callback<()>,
  #[prop_or_default]
  pub dcterm_types: Option<Vec<(String, String)>>,
}

#[component]
pub fn NewAssetPage(props: &NewAssetPageProps) -> Html {
  let tag_line = props
    .tag_line
    .as_ref()
    .map(|tag_line| html!(<p>{ tag_line }</p>))
    .unwrap_or_default();

  html!(
    <>
      <Title level={Level::H2} size={Size::XXXLarge}>{ &props.title }</Title>
      { tag_line }
      <CreateAsset
        on_create={props.on_create.clone()}
        company_name={props.company_name.clone()}
        company_logo_url={props.company_logo_url.clone()}
        dcterm_types={props.dcterm_types.clone()}
      />
    </>
  )
}
