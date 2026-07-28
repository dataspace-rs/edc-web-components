use crate::components::CreateAsset;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct NewAssetPageProps {
  pub on_create: Callback<()>,
}

#[component]
pub fn NewAssetPage(props: &NewAssetPageProps) -> Html {
  html!(
    <>
      <Title level={Level::H2} size={Size::XXXLarge}>{ "New Asset" }</Title>
      <CreateAsset on_create={props.on_create.clone()} />
    </>
  )
}
