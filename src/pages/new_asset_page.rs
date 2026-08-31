use crate::components::CreateAsset;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct NewAssetPageProps {
  #[prop_or_default]
  pub company_name: Option<String>,
  #[prop_or_default]
  pub company_logo_url: Option<String>,
  pub on_create: Callback<()>,
}

#[component]
pub fn NewAssetPage(props: &NewAssetPageProps) -> Html {
  html!(
    <>
      <Title level={Level::H2} size={Size::XXXLarge}>{ "New Asset" }</Title>
      <p>{ "The data you hold, before any of it is offered." }</p>
      <CreateAsset
        on_create={props.on_create.clone()}
        company_name={props.company_name.clone()}
        company_logo_url={props.company_logo_url.clone()}
      />
    </>
  )
}
