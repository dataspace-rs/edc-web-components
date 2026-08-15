use crate::services::{ControlPlaneDspService, DidResolver};
use edc_federated_catalog_client::models::FederatedCatalogParticipantCreateForm;
use edc_federated_catalog_client::{FederatedCatalogClient, FederatedCatalogClientVersion};
use edc_identity_hub_client::models::DidWeb;
//use crate::clients::{FederatedCatalogManagementClient, FederatedCatalogParticipantCreateForm};
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

  let onchange_id = use_callback((id.setter(), target_url.setter()), |value, (id_setter, target_url_setter)| {
    let did = String::from(value);
    let did_val = did.clone();
    let target_setter = target_url_setter.clone();
    spawn_local(async move {

      if let Some(did_web) = DidWeb::new(&did) {
        match DidResolver::new(reqwest::Client::new())
            .resolve(did_web)
            .await
        {
          Ok(data) => {
            if let Some(service) = data.get_identity_services("DataService")
                .first()
                .and_then(|url| Some(ControlPlaneDspService::new(url.service_endpoint.clone())))
                && let Ok(response) = service.get_metadata().await {
              // log::info!("{:?}", service.service_endpoint);
              let endpoint = service.get_dsp_endpoint(response.protocol_versions.first().unwrap().path.clone()).await;
              target_setter.set(endpoint);
            }
          }
          _ => {}
        }
      }
    });
    id_setter.set(did_val);
  });

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
