use crate::components::CreateContractDefinition;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct NewContractDefinitionPageProps {
  #[prop_or("New Contract Definition".to_string())]
  pub title: String,
  #[prop_or(Some("A contract definition links an asset to an access policy and a contract policy. The contract definition constitutes an offer to the other participants.".to_string()))]
  pub tag_line: Option<String>,
  pub on_create: Callback<()>,
}

#[component]
pub fn NewContractDefinitionPage(props: &NewContractDefinitionPageProps) -> Html {
  let tag_line = props
    .tag_line
    .as_ref()
    .map(|tag_line| html!(<p>{ tag_line }</p>))
    .unwrap_or_default();

  html!(
    <>
      <Title level={Level::H2} size={Size::XXXLarge}>{ &props.title }</Title>
      { tag_line }
      <CreateContractDefinition on_create={props.on_create.clone()} />
    </>
  )
}
