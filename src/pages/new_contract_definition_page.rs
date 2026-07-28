use crate::components::CreateContractDefinition;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct NewContractDefinitionPageProps {
  pub on_create: Callback<()>,
}

#[component]
pub fn NewContractDefinitionPage(props: &NewContractDefinitionPageProps) -> Html {
  html!(
    <>
      <Title level={Level::H2} size={Size::XXXLarge}>{ "New Contract Definition" }</Title>
      <CreateContractDefinition on_create={props.on_create.clone()} />
    </>
  )
}
