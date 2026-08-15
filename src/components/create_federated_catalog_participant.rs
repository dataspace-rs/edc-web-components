use crate::services::DidResolver;
use edc_federated_catalog_client::models::FederatedCatalogParticipantCreateForm;
use edc_federated_catalog_client::{FederatedCatalogClient, FederatedCatalogClientVersion};
use edc_identity_hub_client::models::{DidWeb, IdentityServiceType};
use patternfly_yew::prelude::*;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_oauth2::hook::use_latest_access_token;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CreateFederatedCatalogParticipantProps {
  #[prop_or_default]
  pub on_create: Callback<()>,
}

#[component]
pub fn CreateFederatedCatalogParticipant(props: &CreateFederatedCatalogParticipantProps) -> Html {
  let id = use_state(|| "".to_string());
  let name = use_state(|| "".to_string());
  let target_url = use_state(|| "".to_string());

  let onchange_id = use_callback(
    (id.setter(), target_url.setter()),
    |did: String, (id_setter, target_url_setter)| {
      let target_setter = target_url_setter.clone();

      if let Some(did_web) = DidWeb::new(&did) {
        spawn_local(async move {
          if let Ok(identity) = DidResolver::new(reqwest::Client::new())
            .resolve(did_web)
            .await
            && let Some(identity_service) = identity
              .get_identity_services(IdentityServiceType::DataService)
              .first()
            && let Some(dataspace_service_client) = identity_service.get_dataspace_service_client()
            && let Ok(protocol_versions) = dataspace_service_client.get_protocol_versions().await
            && let Some(protocol_versions) = protocol_versions.first()
          {
            target_setter.set(
              identity_service
                .service_endpoint
                .replace("/.well-known/dspace-version", &protocol_versions.path),
            );
          }
        });
      }

      id_setter.set(did);
    },
  );

  let onchange_name = use_callback(name.setter(), |value, name_setter| {
    name_setter.set(value);
  });

  let onchange_target_url = use_callback(target_url.setter(), |value, target_url_setter| {
    target_url_setter.set(value);
  });

  let disabled = false;

  let latest_access_token_context = use_latest_access_token().unwrap();

  let onsubmit = use_callback(
    (
      id.clone(),
      name.clone(),
      target_url.clone(),
      props.on_create.clone(),
      latest_access_token_context.clone(),
    ),
    |event: SubmitEvent, (id, name, target_url, on_create, latest_access_token_context)| {
      event.prevent_default();
      let on_create = on_create.clone();
      let latest_access_token_context = latest_access_token_context.clone();

      let id = (**id).clone();
      let name = (**name).clone();
      let target_url = (**target_url).clone();

      spawn_local(async move {
        let server_url = web_sys::window().unwrap().location().origin().unwrap();
        let federated_catalog_management_client = FederatedCatalogClient::new(
          reqwest::Client::new(),
          format!("{server_url}/federated-catalog-management"),
          latest_access_token_context.access_token(),
          FederatedCatalogClientVersion::V4,
        );

        match federated_catalog_management_client
          .create_participant(&FederatedCatalogParticipantCreateForm {
            id: id.clone(),
            name: name.clone(),
            target_url: target_url.clone(),
          })
          .await
        {
          Ok(()) => {
            on_create.emit(());
          }
          Err(message) => {
            log::error!("{message}");
          }
        }
      })
    },
  );

  html!(
    <Form {onsubmit}>
      <FormGroup label="Counter Party DID" required=true>
        <TextInput required=true value={(*id).to_string()} onchange={onchange_id} />
      </FormGroup>
      <FormGroup label="Counter Party Name" required=true>
        <TextInput required=true value={(*name).to_string()} onchange={onchange_name} />
      </FormGroup>
      <FormGroup label="Counter Party Address" required=true>
        <TextInput
          required=true
          value={(*target_url).to_string()}
          onchange={onchange_target_url}
          r#type={TextInputType::Url}
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
