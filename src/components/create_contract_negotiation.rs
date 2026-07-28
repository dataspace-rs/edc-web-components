use crate::components::PolicySelector;
use crate::contexts::use_edc_connector_context;
use crate::models::PolicyDefinitionItem;
use edc_connector_client::EdcConnectorApiVersion;
use edc_connector_client::types::contract_negotiation::ContractRequest;
use patternfly_yew::prelude::*;
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CreateContractNegotiationProps {
  #[prop_or_default]
  pub on_create: Callback<()>,
}

#[component]
pub fn CreateContractNegotiation(props: &CreateContractNegotiationProps) -> Html {
  let edc_connector_context = use_edc_connector_context();

  let counter_party_address = use_state(String::new);
  let counter_party_id = use_state(String::new);
  let policy_definition_item = use_state(|| Option::<PolicyDefinitionItem>::None);

  let onsubmit = use_callback(
    (
      edc_connector_context.clone(),
      counter_party_address.clone(),
      counter_party_id.clone(),
      policy_definition_item.clone(),
      props.on_create.clone(),
    ),
    |event: SubmitEvent,
     (
      edc_connector_context,
      counter_party_address,
      counter_party_id,
      policy_definition_item,
      oncreate,
    )| {
      event.prevent_default();

      let edc_connector_context = edc_connector_context.clone();
      let counter_party_address = (**counter_party_address).clone();
      let counter_party_id = (**counter_party_id).clone();
      let policy_definition_item = (**policy_definition_item).clone();
      let oncreate = oncreate.clone();

      spawn_local(async move {
        if let Some(policy_definition_item) = policy_definition_item
          && let Some(client) = edc_connector_context.get_client()
          && let Ok(policy_definition) = client
            .policies(EdcConnectorApiVersion::V4)
            .get(&policy_definition_item.id)
            .await
        {
          let contract_request = ContractRequest::builder()
            .counter_party_address(counter_party_address)
            .counter_party_id(counter_party_id)
            .policy(policy_definition.policy().clone())
            .build();

          let _ = client
            .contract_negotiations(EdcConnectorApiVersion::V4)
            .initiate(&contract_request)
            .await;

          oncreate.emit(());
        }
      })
    },
  );

  let onchange_counter_party_address = use_callback(
    counter_party_address.setter(),
    move |counter_party_address, counter_party_address_setter| {
      counter_party_address_setter.set(counter_party_address);
    },
  );

  let onchange_counter_party_id = use_callback(
    counter_party_id.setter(),
    move |counter_party_id, counter_party_id_setter| {
      counter_party_id_setter.set(counter_party_id);
    },
  );

  let onselect_policy = use_callback(
    policy_definition_item.setter(),
    move |policy_definition_item, policy_definition_item_setter| {
      policy_definition_item_setter.set(Some(policy_definition_item));
    },
  );

  let disabled = (*policy_definition_item).is_none();

  html!(
    <Form {onsubmit}>
      <FormGroup label="Counter Party Address" required=true>
        <TextInput
          required=true
          value={(*counter_party_address).to_string()}
          onchange={onchange_counter_party_address}
        />
      </FormGroup>
      <FormGroup label="Counter Party ID" required=true>
        <TextInput
          required=true
          value={(*counter_party_id).to_string()}
          onchange={onchange_counter_party_id}
        />
      </FormGroup>
      <FormGroup label="Policy" required=true>
        <PolicySelector
          onselect={onselect_policy}
          selected_policy={(*policy_definition_item).clone()}
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
