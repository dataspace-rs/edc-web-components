use crate::components::{
  AssetReference, ContractAgreementReference, ContractNegotiationStatus, DidLabel,
};
use crate::contexts::use_edc_connector_context;
#[cfg(feature = "contract-negotiation-review")]
use edc_connector_client::types::contract_negotiation::{
  ContractNegotiationKind, ContractNegotiationState,
};
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ShowContractNegotiationPageProps {
  #[prop_or("Contract Negotiation".to_string())]
  pub title: String,
  #[prop_or(None)]
  pub tag_line: Option<String>,
  pub contract_negotiation_id: String,
  #[prop_or_default]
  pub on_contract_agreement_click: Callback<String>,
}

#[component]
pub fn ShowContractNegotiationPage(props: &ShowContractNegotiationPageProps) -> Html {
  let tag_line = props
    .tag_line
    .as_ref()
    .map(|tag_line| html!(<p>{ tag_line }</p>))
    .unwrap_or_default();

  html!(
    <>
      <Title level={Level::H2} size={Size::XXXLarge}>{ &props.title }</Title>
      { tag_line }
      <Suspense fallback={html! {<Bullseye><Spinner /></Bullseye>}}>
        <ShowContractNegotiationPageInner
          contract_negotiation_id={props.contract_negotiation_id.clone()}
          on_contract_agreement_click={props.on_contract_agreement_click.clone()}
        />
      </Suspense>
    </>
  )
}

#[component]
pub fn ShowContractNegotiationPageInner(props: &ShowContractNegotiationPageProps) -> HtmlResult {
  let edc_connector_client = use_edc_connector_context();

  let refresh = use_state(|| 0usize);

  let contract_negotiation = use_future_with(
    (
      props.contract_negotiation_id.clone(),
      edc_connector_client.clone(),
      refresh.clone(),
    ),
    |properties| async move {
      let (contract_negotiation_id, edc_connector_client, _) = (*properties).clone();

      if let Some(client) = edc_connector_client.get_client() {
        client
          .contract_negotiations(edc_connector_client::EdcConnectorApiVersion::V4)
          .get(&contract_negotiation_id)
          .await
          .ok()
      } else {
        None
      }
    },
  )?;

  let on_finalized = use_callback(refresh.setter(), |_, refresh_setter| {
    refresh_setter.set(1);
  });

  #[cfg(feature = "contract-negotiation-review")]
  let on_reviewed = use_callback(refresh.setter(), |_, refresh_setter| {
    refresh_setter.set(1);
  });

  let contract_negotiation = (*contract_negotiation).clone();

  if let Some(contract_negotiation) = contract_negotiation {
    let contract_agreement = contract_negotiation
      .contract_agreement_id()
      .map(|contract_agreement_id| {
        html!(
          <DescriptionGroup term="Contract Agreement">
            <ContractAgreementReference contract_agreement_id={contract_agreement_id.clone()} />
          </DescriptionGroup>
        )
      })
      .unwrap_or_default();

    let state =
      Some(crate::models::ContractNegotiationState::from(contract_negotiation.state()).to_string())
        .map(|value| {
          let color = match value.as_str() {
            "Finalized" => Color::Green,
            "Terminated" => Color::Red,
            _ => Color::Blue,
          };

          html!(
            <DescriptionGroup term="State">
              <Label label={value} {color} />
            </DescriptionGroup>
          )
        });

    let kind =
      crate::models::ContractNegotiationKind::from(contract_negotiation.kind()).to_string();

    #[cfg(feature = "contract-negotiation-review")]
    let review = if contract_negotiation.state() == &ContractNegotiationState::Requested
      && contract_negotiation.kind() == &ContractNegotiationKind::Provider
    {
      let contract_negotiation_id = contract_negotiation.id().to_string();

      html!(
        <Alert title="Review the Contract Negotiation">
          <Suspense fallback={html!(<Bullseye><Spinner /></Bullseye>)}>
            <crate::components::ReviewContractNegotiation {contract_negotiation_id} {on_reviewed} />
          </Suspense>
        </Alert>
      )
    } else {
      html!()
    };

    #[cfg(not(feature = "contract-negotiation-review"))]
    let review = html!();

    Ok(html!(
      <Stack gutter=true>
        <StackItem>
          <DescriptionList mode={[DescriptionListMode::Horizontal]}>
            <DescriptionGroup term="Id">{ contract_negotiation.id() }</DescriptionGroup>
            <DescriptionGroup term="Counter Party">
              <DidLabel did={contract_negotiation.counter_party_id().clone().unwrap_or_default()} />
            </DescriptionGroup>
            <DescriptionGroup term="Asset">
              <AssetReference
                asset_id={contract_negotiation.asset_id().clone().unwrap_or_default()}
              />
            </DescriptionGroup>
            { state }
            <DescriptionGroup term="Kind">{ kind }</DescriptionGroup>
            { contract_agreement }
          </DescriptionList>
        </StackItem>
        <StackItem>
          <ContractNegotiationStatus
            contract_negotiation_id={props.contract_negotiation_id.clone()}
            {on_finalized}
          />
        </StackItem>
        <StackItem>{ review }</StackItem>
      </Stack>
    ))
  } else {
    Ok(html!(
      format!(
      "Contract Negotiation with id {} not found.",
      props.contract_negotiation_id
    )
    ))
  }
}
