use crate::components::CreatePolicy;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct NewPolicyPageProps {
  pub on_create: Callback<()>,
}

#[component]
pub fn NewPolicyPage(props: &NewPolicyPageProps) -> Html {
  html!(
    <>
      <Title level={Level::H2} size={Size::XXXLarge}>{ "New Policy" }</Title>
      <CreatePolicy on_create={props.on_create.clone()} />
    </>
  )
}
