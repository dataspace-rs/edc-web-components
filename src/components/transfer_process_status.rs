use crate::contexts::use_edc_connector_context;
use edc_connector_client::types::transfer_process::{TransferProcess, TransferProcessState};
use patternfly_yew::prelude::*;
use std::time::Duration;
use yew::platform::spawn_local;
use yew::platform::time::sleep;
use yew::prelude::*;

const HAPPY_PATH_STATES: [(&str, TransferProcessState); 5] = [
  ("Initial", TransferProcessState::Initial),
  ("Provisioned", TransferProcessState::Provisioned),
  ("Requested", TransferProcessState::Requested),
  ("Started", TransferProcessState::Started),
  ("Completed", TransferProcessState::Completed),
];

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TransferProcessStatusProps {
  pub transfer_process_id: String,
  #[prop_or_default]
  pub on_started: Callback<()>,
  #[prop_or_default]
  pub on_finalized: Callback<()>,
}

#[component]
pub fn TransferProcessStatus(props: &TransferProcessStatusProps) -> Html {
  let edc_connector_client = use_edc_connector_context();

  let transfer_process_state = use_state(|| None);
  let transfer_process_error = use_state(|| Option::<String>::None);

  use_effect_with(
    (
      props.transfer_process_id.clone(),
      edc_connector_client.clone(),
      transfer_process_state.setter(),
      transfer_process_error.setter(),
      props.on_finalized.clone(),
      props.on_started.clone(),
    ),
    |(
      transfer_process_id,
      edc_connector_client,
      transfer_process_state_setter,
      transfer_process_error_setter,
      on_finalized,
      on_started,
    )| {
      let transfer_process_id = transfer_process_id.clone();
      let edc_connector_client = edc_connector_client.clone();
      let transfer_process_state_setter = transfer_process_state_setter.clone();
      let transfer_process_error_setter = transfer_process_error_setter.clone();
      let on_finalized = on_finalized.clone();
      let on_started = on_started.clone();

      spawn_local(async move {
        loop {
          let transfer_process = if let Some(client) = edc_connector_client.get_client() {
            client
              .transfer_processes(edc_connector_client::EdcConnectorApiVersion::V4)
              .get(&transfer_process_id)
              .await
              .ok()
          } else {
            None
          };

          let state = transfer_process
            .as_ref()
            .map(|transfer_process| transfer_process.state().clone());

          let error_message = transfer_process.as_ref().and_then(|transfer_process| {
            transfer_process
              .error_detail()
              .map(|error_detail| error_detail.to_string())
          });

          transfer_process_error_setter.set(error_message);

          transfer_process_state_setter.set(transfer_process);

          if state == Some(TransferProcessState::Started) {
            on_started.emit(());
            break;
          }

          if state == Some(TransferProcessState::Completed)
            || state == Some(TransferProcessState::Terminated)
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
    (*transfer_process_error).clone(),
    (*transfer_process_state).clone(),
  ) {
    (Some(error_message), _) => html!(
      <Alert title="Transfer Process failed" r#type={AlertType::Danger}>
        <div>{ error_message }</div>
      </Alert>
    ),
    (None, Some(transfer_process)) => {
      let transfer_process: TransferProcess = transfer_process;

      let current_state_index = HAPPY_PATH_STATES
        .iter()
        .position(|(_, state)| state == transfer_process.state())
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
