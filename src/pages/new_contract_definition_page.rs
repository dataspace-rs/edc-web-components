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
      <p>
        { "A contract definition links an asset to an access policy and a contract policy. The contract definition constitutes an offer to the other participants." }
      </p>
      <CreateContractDefinition on_create={props.on_create.clone()} />
    </>
  )
}
