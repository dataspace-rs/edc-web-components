use crate::contexts::use_edc_connector_context;
use edc_connector_client::EdcConnectorApiVersion;
use edc_connector_client::types::transfer_process::TransferRequest;
use patternfly_yew::prelude::*;
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CreateTransferProcessProps {
  #[prop_or_default]
  pub on_create: Callback<()>,
}

#[component]
pub fn CreateTransferProcess(props: &CreateTransferProcessProps) -> Html {
  let edc_connector_context = use_edc_connector_context();

  let contract_id = use_state(|| "".to_string());
  let counter_party_address = use_state(|| "".to_string());
  let transfer_type = use_state(|| "".to_string());

  let onsubmit = use_callback(
    (
      edc_connector_context,
      contract_id.clone(),
      counter_party_address.clone(),
      transfer_type.clone(),
      props.on_create.clone(),
    ),
    |event: SubmitEvent,
     (edc_connector_context, contract_id, counter_party_address, transfer_type, on_create)| {
      event.prevent_default();

      let edc_connector_context = edc_connector_context.clone();
      let contract_id = (**contract_id).clone();
      let counter_party_address = (**counter_party_address).clone();
      let transfer_type = (**transfer_type).clone();
      let on_create = on_create.clone();

      spawn_local(async move {
        let transfer_request = TransferRequest::builder()
          .contract_id(contract_id)
          .counter_party_address(counter_party_address)
          .transfer_type(transfer_type)
          .build();

        if let Some(client) = edc_connector_context.get_client() {
          let _ = client
            .transfer_processes(EdcConnectorApiVersion::V4)
            .initiate(&transfer_request)
            .await;

          on_create.emit(());
        }
      })
    },
  );

  let onchange_contract_id = use_callback(
    contract_id.setter(),
    move |contract_id, contract_id_setter| {
      contract_id_setter.set(contract_id);
    },
  );

  let onchange_counter_party_address = use_callback(
    counter_party_address.setter(),
    move |counter_party_address, counter_party_address_setter| {
      counter_party_address_setter.set(counter_party_address);
    },
  );

  let onchange_transfer_type = use_callback(
    transfer_type.setter(),
    move |transfer_type, transfer_type_setter| {
      transfer_type_setter.set(transfer_type);
    },
  );

  let disabled = false;

  html!(
    <Form {onsubmit}>
      <FormGroup label="Contract ID" required=true>
        <TextInput
          required=true
          value={(*contract_id).to_string()}
          onchange={onchange_contract_id}
        />
      </FormGroup>
      <FormGroup label="Counter Party Address" required=true>
        <TextInput
          required=true
          value={(*counter_party_address).to_string()}
          onchange={onchange_counter_party_address}
        />
      </FormGroup>
      <FormGroup label="Transfer Type" required=true>
        <TextInput
          required=true
          value={(*transfer_type).to_string()}
          onchange={onchange_transfer_type}
        />
      </FormGroup>
      <ActionGroup>
        <Button
          variant={ButtonVariant::Primary}
          label="Submit"
          r#type={ButtonType::Submit}
          {disabled}
        />
        <Button variant={ButtonVariant::Secondary} label="Reset" r#type={ButtonType::Reset} />
      </ActionGroup>
    </Form>
  )
}
