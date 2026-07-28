use crate::components::{AssetSelector, PolicySelector};
use crate::contexts::use_edc_connector_context;
use crate::models::{AssetItem, PolicyDefinitionItem};
use edc_connector_client::EdcConnectorApiVersion;
use edc_connector_client::types::contract_definition::NewContractDefinition;
use edc_connector_client::types::query::Criterion;
use patternfly_yew::prelude::*;
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CreateContractDefinitionProps {
  #[prop_or_default]
  pub on_create: Callback<()>,
}

#[component]
pub fn CreateContractDefinition(props: &CreateContractDefinitionProps) -> Html {
  let edc_connector_context = use_edc_connector_context();

  let identifier = use_state(String::new);
  let access_policy_definition_item = use_state(|| Option::<PolicyDefinitionItem>::None);
  let contract_policy_definition_item = use_state(|| Option::<PolicyDefinitionItem>::None);
  let asset_items = use_state(Vec::<AssetItem>::new);

  let onsubmit = use_callback(
    (
      edc_connector_context.clone(),
      identifier.clone(),
      access_policy_definition_item.clone(),
      contract_policy_definition_item.clone(),
      asset_items.clone(),
      props.on_create.clone(),
    ),
    |event: SubmitEvent,
     (
      edc_connector_context,
      identifier,
      access_policy_definition_item,
      contract_policy_definition_item,
      asset_items,
      on_create,
    )| {
      event.prevent_default();

      let edc_connector_context = edc_connector_context.clone();
      let identifier = (**identifier).clone();
      let access_policy_definition_item = (**access_policy_definition_item).clone();
      let contract_policy_definition_item = (**contract_policy_definition_item).clone();
      let asset_items = (**asset_items).clone();
      let on_create = on_create.clone();

      spawn_local(async move {
        let mut new_contract_definition = NewContractDefinition::builder()
          .id(&identifier)
          .access_policy_id(
            access_policy_definition_item
              .map(|policy_definition_item| policy_definition_item.id)
              .unwrap_or_default(),
          )
          .contract_policy_id(
            contract_policy_definition_item
              .map(|policy_definition_item| policy_definition_item.id.to_string())
              .unwrap_or_default(),
          );

        for asset_item in &asset_items {
          new_contract_definition = new_contract_definition.asset_selector(Criterion::new(
            "https://w3id.org/edc/v0.0.1/ns/id",
            "=",
            asset_item.id.clone(),
          ));
        }

        let new_contract_definition = new_contract_definition.build();

        if let Some(client) = edc_connector_context.get_client() {
          let _ = client
            .contract_definitions(EdcConnectorApiVersion::V4)
            .create(&new_contract_definition)
            .await;

          on_create.emit(());
        }
      })
    },
  );

  let onchange_identifier =
    use_callback(identifier.setter(), move |identifier, identifier_setter| {
      identifier_setter.set(identifier);
    });

  let onselect_access_policy = use_callback(
    access_policy_definition_item.setter(),
    move |access_policy_definition_item, access_policy_definition_item_setter| {
      access_policy_definition_item_setter.set(Some(access_policy_definition_item));
    },
  );

  let onselect_contract_policy = use_callback(
    contract_policy_definition_item.setter(),
    move |contract_policy, contract_policy_setter| {
      contract_policy_setter.set(Some(contract_policy));
    },
  );

  let disabled = (*identifier).is_empty()
    || (*access_policy_definition_item).is_none()
    || (*contract_policy_definition_item).is_none();

  let onselect_assets = use_callback(asset_items.setter(), |asset_items, asset_items_setter| {
    asset_items_setter.set(asset_items);
  });

  html!(
    <Form {onsubmit}>
      <FormGroup label="Identifier" required=true>
        <TextInput required=true value={(*identifier).to_string()} onchange={onchange_identifier} />
      </FormGroup>
      <FormGroup label="Access Policy" required=true>
        <PolicySelector
          onselect={onselect_access_policy}
          selected_policy={(*access_policy_definition_item).clone()}
          select_id="selectable-access-policy"
        />
      </FormGroup>
      <FormGroup label="Contract Policy" required=true>
        <PolicySelector
          onselect={onselect_contract_policy}
          selected_policy={(*contract_policy_definition_item).clone()}
          select_id="selectable-contract-policy"
        />
      </FormGroup>
      <FormGroup label="Asset Selector">
        <AssetSelector onselect={onselect_assets} selected_assets={(*asset_items).clone()} />
      </FormGroup>
      <ActionGroup>
        <Button
          variant={ButtonVariant::Primary}
          label="Submit"
          r#type={ButtonType::Submit}
          {disabled}
        />
      </ActionGroup>
    </Form>
  )
}
