use crate::components::ContractNegotiationStatus;
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
  pub contract_negotiation_id: String,
  #[prop_or_default]
  pub on_contract_agreement_click: Callback<String>,
}

#[component]
pub fn ShowContractNegotiationPage(props: &ShowContractNegotiationPageProps) -> Html {
  html!(
    <>
      <Title level={Level::H2} size={Size::XXXLarge}>{ "Contract Negotiation" }</Title>
      <Suspense fallback="Loading ...">
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
    let contract_agreement =
      if let Some(contract_agreement_id) = contract_negotiation.contract_agreement_id() {
        let onclick = {
          let contract_agreement_id = contract_agreement_id.clone();

          props
            .on_contract_agreement_click
            .reform(move |_| contract_agreement_id.clone())
        };

        html!(
          <DescriptionGroup term="Contract Agreement Id">
            <Button variant={ButtonVariant::InlineLink} {onclick}>{ contract_agreement_id }</Button>
          </DescriptionGroup>
        )
      } else {
        html!()
      };

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
            { state }
            <DescriptionGroup term="Kind">{ kind }</DescriptionGroup>
            { contract_agreement }
            <DescriptionGroup term="Counter Party ID">
              { contract_negotiation.counter_party_id().clone().unwrap_or_default() }
            </DescriptionGroup>
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
