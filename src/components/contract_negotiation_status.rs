use crate::contexts::use_edc_connector_context;
use edc_connector_client::types::contract_negotiation::{
  ContractNegotiation, ContractNegotiationState,
};
use patternfly_yew::prelude::*;
use std::time::Duration;
use yew::platform::spawn_local;
use yew::platform::time::sleep;
use yew::prelude::*;

const HAPPY_PATH_STATES: [(&str, ContractNegotiationState); 7] = [
  ("Initial", ContractNegotiationState::Initial),
  ("Requested", ContractNegotiationState::Requested),
  ("Offered", ContractNegotiationState::Offered),
  ("Accepted", ContractNegotiationState::Accepted),
  ("Agreed", ContractNegotiationState::Agreed),
  ("Verified", ContractNegotiationState::Verified),
  ("Finalized", ContractNegotiationState::Finalized),
];

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ContractNegotiationStatusProps {
  pub contract_negotiation_id: String,
  pub on_finalized: Callback<()>,
}

#[component]
pub fn ContractNegotiationStatus(props: &ContractNegotiationStatusProps) -> Html {
  let edc_connector_client = use_edc_connector_context();

  let contract_negotiation_state = use_state(|| None);
  let contract_negotiation_error = use_state(|| None);

  use_effect_with(
    (
      props.contract_negotiation_id.clone(),
      edc_connector_client.clone(),
      contract_negotiation_state.setter(),
      contract_negotiation_error.setter(),
      props.on_finalized.clone(),
    ),
    |(
      contract_negotiation_id,
      edc_connector_client,
      contract_negotiation_state_setter,
      contract_negotiation_error_setter,
      on_finalized,
    )| {
      let contract_negotiation_id = contract_negotiation_id.clone();
      let edc_connector_client = edc_connector_client.clone();
      let contract_negotiation_state_setter = contract_negotiation_state_setter.clone();
      let contract_negotiation_error_setter = contract_negotiation_error_setter.clone();
      let on_finalized = on_finalized.clone();

      spawn_local(async move {
        loop {
          let contract_negotiation = if let Some(client) = edc_connector_client.get_client() {
            client
              .contract_negotiations(edc_connector_client::EdcConnectorApiVersion::V4)
              .get(&contract_negotiation_id)
              .await
              .ok()
          } else {
            None
          };

          let state = contract_negotiation
            .as_ref()
            .map(|contract_negotiation| contract_negotiation.state().clone());

          let error_message = contract_negotiation
            .as_ref()
            .and_then(|contract_negotiation| {
              contract_negotiation
                .error_detail()
                .map(|error_detail| error_detail.to_string())
            });

          contract_negotiation_error_setter.set(error_message);

          contract_negotiation_state_setter.set(contract_negotiation);

          if state == Some(ContractNegotiationState::Finalized)
            || state == Some(ContractNegotiationState::Terminated)
          {
            on_finalized.emit(());
            break;
          }

          sleep(Duration::from_secs(1)).await;
        }
      });
    },
  );

  match (
    (*contract_negotiation_error).clone(),
    (*contract_negotiation_state).clone(),
  ) {
    (Some(error_message), _) => html!(
      <Alert title="Negotiation failed" r#type={AlertType::Danger}>
        <div>{ error_message }</div>
      </Alert>
    ),
    (None, Some(contract_negotiation)) => {
      let contract_negotiation: ContractNegotiation = contract_negotiation;

      let current_state_index = HAPPY_PATH_STATES
        .iter()
        .position(|(_, state)| state == contract_negotiation.state())
        .unwrap_or_default();

      let steps = HAPPY_PATH_STATES
        .iter()
        .enumerate()
        .map(|(index, (state_label, _))| {
          let status = if index < current_state_index {
            ProgressStepperStepStatus::Success
          } else if index == current_state_index {
            if index == HAPPY_PATH_STATES.len() - 1 {
              ProgressStepperStepStatus::Success
            } else {
              ProgressStepperStepStatus::Default
            }
          } else {
            ProgressStepperStepStatus::Pending
          };

          html_nested!(
            <ProgressStepperStep {status}>
              <div>{ state_label.to_string() }</div>
            </ProgressStepperStep>
          )
        });

      html!(<ProgressStepper>{ for steps }</ProgressStepper>)
    }
    _ => html!(),
  }
}
