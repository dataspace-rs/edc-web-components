use crate::contexts::use_edc_identity_hub_context;
use edc_identity_hub_client::IdentityHubClient;
use edc_identity_hub_client::models::{
  CredentialFormat, CredentialQuery, CredentialsSupported, DidWeb, RequestCredentialBody,
  RequestCredentialState,
};
use patternfly_yew::prelude::*;
use uuid::Uuid;
use yew::platform::spawn_local;
use yew::platform::time::sleep;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Issuer {
  pub did: String,
  pub name: String,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct RequestVerifiableCredentialProps {
  #[prop_or_default]
  pub issuers: Vec<Issuer>,
  #[prop_or_default]
  pub on_create: Callback<()>,
}

#[component]
pub fn RequestVerifiableCredential(props: &RequestVerifiableCredentialProps) -> Html {
  let requesting = use_state(|| false);
  let issuer_did = use_state(String::new);
  let credentials_supported = use_state(Vec::new);

  let selected_credentials_supported = use_state(Vec::<CredentialsSupported>::new);

  let onchange_issuer_did = use_callback(
    (issuer_did.setter(), credentials_supported.setter()),
    move |issuer_did: String, (issuer_did_setter, credentials_supported_setter)| {
      issuer_did_setter.set(issuer_did.clone());
      credentials_supported_setter.set(vec![]);
      let credentials_supported_setter = credentials_supported_setter.clone();

      spawn_local(async move {
        if let Some(issuer_did_web) = DidWeb::new(&issuer_did)
          && let Ok(identity) =
            IdentityHubClient::get_identity(reqwest::Client::new(), issuer_did_web).await
        {
          log::debug!("Identity: {:?}", identity);
          if let Some(issuer_service_client) = identity
            .get_identity_services("IssuerService")
            .first()
            .and_then(|identity_service| identity_service.get_issuer_service_client())
            && let Ok(issuer_service_metadata) = issuer_service_client.get_metadata().await
          {
            log::debug!("Issuer Service Metadata: {:?}", issuer_service_metadata);
            credentials_supported_setter.set(issuer_service_metadata.credentials_supported);
          }
        }
      })
    },
  );

  let edc_identity_hub_context = use_edc_identity_hub_context();

  let onsubmit = use_callback(
    (
      issuer_did.clone(),
      selected_credentials_supported.clone(),
      props.on_create.clone(),
      edc_identity_hub_context.clone(),
      requesting.clone(),
    ),
    |event: SubmitEvent,
     (
      issuer_did,
      selected_credentials_supported,
      on_create,
      edc_identity_hub_context,
      requesting,
    )| {
      event.prevent_default();
      let edc_identity_hub_context = edc_identity_hub_context.clone();
      let on_create = on_create.clone();
      let selected_credentials_supported = (**selected_credentials_supported).clone();
      let issuer_did = (**issuer_did).clone();
      let requesting = requesting.clone();

      spawn_local(async move {
        requesting.set(true);
        let holder_pid = Uuid::new_v4().to_string();

        let credentials = selected_credentials_supported
          .iter()
          .map(|credential| CredentialQuery {
            format: CredentialFormat::Vc10Jwt,
            r#type: credential.credential_type.clone(),
            id: credential.id.clone(),
          })
          .collect::<Vec<_>>();

        let body = RequestCredentialBody {
          issuer_did,
          holder_pid: holder_pid.clone(),
          credentials,
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

          match edc_identity_hub_context
            .get_client()
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

  let disabled =
    (*issuer_did).is_empty() || (*selected_credentials_supported).is_empty() || *requesting;

  let issuer_field = if props.issuers.is_empty() {
    html!(
      <TextInput
        required=true
        value={(*issuer_did).to_string()}
        onchange={onchange_issuer_did.clone()}
      />
    )
  } else {
    let options = props.issuers.iter().map(|issuer| {
      let did = issuer.did.clone();

      html_nested!(
        <MenuAction
          onclick={onchange_issuer_did.reform(move |_| did.clone())}
          selected={issuer.did == *issuer_did}
        >
          <DescriptionList>
            <DescriptionGroup term={issuer.name.clone()}>
              <small>{ issuer.did.clone() }</small>
            </DescriptionGroup>
          </DescriptionList>
        </MenuAction>
      )
    });

    let label = props
      .issuers
      .iter()
      .find(|issuer| issuer.did == *issuer_did)
      .map(|issuer| issuer.name.clone())
      .unwrap_or("Select an issuer".to_string());

    html!(<Dropdown text={label}>{ for options }</Dropdown>)
  };

  let onselected = use_callback(
    selected_credentials_supported.clone(),
    |(checkbox_state, credentials_supported): (CheckboxState, CredentialsSupported),
     selected_credentials_supported| {
      let mut list = (**selected_credentials_supported).clone();

      if checkbox_state == CheckboxState::Checked {
        list.push(credentials_supported)
      } else {
        list.retain(|credential| *credential != credentials_supported);
      }

      selected_credentials_supported.set(list);
    },
  );

  let credentials_supported = credentials_supported.iter().map(|credential| {
    let checked = if (*selected_credentials_supported).contains(credential) { CheckboxState::Checked } else { CheckboxState::Unchecked };
    let credential_type = credential.credential_type.clone();
    let credential = credential.clone();

    html!(
      <Card selectable=true>
        <CardHeader
          selectable_actions={yew::props!(CardSelectableActionsObjectProperties {
                            action:CardSelectableActionsVariant::MultiSelect {
                                checked,
                                onchange: onselected.reform(move |checkbox_state| (checkbox_state, credential.clone())),
                            },
                            base: yew::props!(CardSelectableActionsObjectBase {
                                name: "clickable-credentials"
                            })
                        })}
        >
          { credential_type }
        </CardHeader>
      </Card>
    )
  });

  html!(
    <Form {onsubmit}>
      <FormGroup label="Issuer" required=true>{ issuer_field }</FormGroup>
      <FormGroup label="Credential Type" required=true>{ for credentials_supported }</FormGroup>
      <ActionGroup>
        <Button
          variant={ButtonVariant::Primary}
          label="Generate my Verifiable Credential"
          r#type={ButtonType::Submit}
          {disabled}
        />
      </ActionGroup>
    </Form>
  )
}
