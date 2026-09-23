use crate::components::CreateTransferProcess;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct NewTransferProcessPageProps {
  #[prop_or("New Transfer Process".to_string())]
  pub title: String,
  #[prop_or(None)]
  pub tag_line: Option<String>,
  #[prop_or_default]
  pub contract_agreement_id: Option<String>,
  #[prop_or_default]
  pub counter_party_address: Option<String>,
  pub on_create: Callback<String>,
}

#[component]
pub fn NewTransferProcessPage(props: &NewTransferProcessPageProps) -> Html {
  let tag_line = props
    .tag_line
    .as_ref()
    .map(|tag_line| html!(<p>{ tag_line }</p>))
    .unwrap_or_default();

  html!(
    <>
      <Title level={Level::H2} size={Size::XXXLarge}>{ &props.title }</Title>
      { tag_line }
      <CreateTransferProcess
        contract_agreement_id={props.contract_agreement_id.clone()}
        counter_party_address={props.counter_party_address.clone()}
        on_create={props.on_create.clone()}
      />
    </>
  )
}
