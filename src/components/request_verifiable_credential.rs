use crate::contexts::use_edc_identity_hub_context;
use edc_identity_hub_client::models::{
  CredentialFormat, CredentialQuery, RequestCredentialBody, RequestCredentialState,
};
use patternfly_yew::prelude::*;
use uuid::Uuid;
use yew::platform::spawn_local;
use yew::platform::time::sleep;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct RequestVerifiableCredentialProps {
  #[prop_or_default]
  pub on_create: Callback<()>,
}

#[component]
pub fn RequestVerifiableCredential(props: &RequestVerifiableCredentialProps) -> Html {
  let requesting = use_state(|| false);
  let issuer_did = use_state(String::new);
  let holder_pid = use_state(|| Uuid::new_v4().to_string());
  let r#type = use_state(String::new);
  let id = use_state(String::new);

  let onchange_issuer_did = use_callback(
    issuer_did.setter(),
    move |issuer_did: String, issuer_did_setter| {
      issuer_did_setter.set(issuer_did);
    },
  );

  let onchange_holder_pid = use_callback(
    holder_pid.setter(),
    move |holder_pid: String, holder_pid_setter| {
      holder_pid_setter.set(holder_pid);
    },
  );

  let onchange_type = use_callback(r#type.setter(), move |r#type: String, type_setter| {
    type_setter.set(r#type);
  });

  let onchange_id = use_callback(id.setter(), move |id: String, id_setter| {
    id_setter.set(id);
  });

  let edc_identity_hub_context = use_edc_identity_hub_context();

  let onsubmit = use_callback(
    (
      issuer_did.clone(),
      holder_pid.clone(),
      r#type.clone(),
      id.clone(),
      props.on_create.clone(),
      edc_identity_hub_context.clone(),
      requesting.clone(),
    ),
    |event: SubmitEvent,
     (issuer_did, holder_pid, r#type, id, on_create, edc_identity_hub_context, requesting)| {
      event.prevent_default();
      let edc_identity_hub_context = edc_identity_hub_context.clone();
      let on_create = on_create.clone();
      let issuer_did = (**issuer_did).clone();
      let holder_pid = (**holder_pid).clone();
      let r#type = (**r#type).clone();
      let id = (**id).clone();
      let requesting = requesting.clone();

      spawn_local(async move {
        requesting.set(true);

        let body = RequestCredentialBody {
          issuer_did,
          holder_pid: holder_pid.clone(),
          credentials: vec![CredentialQuery {
            format: CredentialFormat::Vc10Jwt,
            r#type,
            id,
          }],
        };

        let identity_hub_client = edc_identity_hub_context.get_client();

        if let Err(error) = identity_hub_client
          .request_verifiable_credential(edc_identity_hub_context.participant_id(), &body)
          .await
        {
          log::error!("Error requesting verifiable credential: {:?}", error);
        }

        loop {
          sleep(std::time::Duration::from_millis(500)).await;
          log::info!("Waiting for verifiable credential status to be updated...");

          match identity_hub_client
            .get_request_verifiable_credential_status(
              edc_identity_hub_context.participant_id(),
              &holder_pid,
            )
            .await
          {
            Ok(status) if status.status == RequestCredentialState::Issued => {
              break;
            }
            Ok(status) if status.status == RequestCredentialState::Error => {
              break;
            }
            _ => {}
          }
        }

        on_create.emit(());
        requesting.set(false);
      });
    },
  );

  let disabled = (*issuer_did).is_empty()
    || (*holder_pid).is_empty()
    || (*r#type).is_empty()
    || (*id).is_empty()
    || *requesting;

  html!(
    <Form {onsubmit}>
      <FormGroup label="Issuer DID" required=true>
        <TextInput required=true value={(*issuer_did).to_string()} onchange={onchange_issuer_did} />
      </FormGroup>
      <FormGroup label="Holder PID" required=true>
        <TextInput required=true value={(*holder_pid).to_string()} onchange={onchange_holder_pid} />
      </FormGroup>
      <FormGroup label="Credential Type" required=true>
        <TextInput required=true value={(*r#type).to_string()} onchange={onchange_type} />
      </FormGroup>
      <FormGroup label="Credential ID" required=true>
        <TextInput required=true value={(*id).to_string()} onchange={onchange_id} />
      </FormGroup>
      <ActionGroup>
        <Button
          variant={ButtonVariant::Primary}
          label="Submit"
          r#type={ButtonType::Submit}
          {disabled}
        />
        <Button
          variant={ButtonVariant::Secondary}
          label="Reset"
          r#type={ButtonType::Reset}
          disabled={*requesting}
        />
      </ActionGroup>
    </Form>
  )
}
