use edc_identity_hub_client::{IdentityHubClient, IdentityHubClientVersion};
use yew::prelude::*;
use yew_oauth2::context::LatestAccessToken;
use yew_oauth2::prelude::use_latest_access_token;

#[derive(Clone, PartialEq)]
pub struct EdcIdentityHubState {
  participant_id: String,
  participant_did: String,
  latest_access_token_context: LatestAccessToken,
}

impl EdcIdentityHubState {
  pub fn get_client(&self) -> IdentityHubClient {
    let client = reqwest::Client::new();

    let server_url = web_sys::window().unwrap().location().origin().unwrap();

    IdentityHubClient::new(
      client,
      format!("{server_url}/identity-hub"),
      self.latest_access_token_context.access_token(),
      IdentityHubClientVersion::V1Alpha,
    )
  }

  pub fn participant_id(&self) -> &str {
    &self.participant_id
  }
  pub fn participant_did(&self) -> &str {
    &self.participant_did
  }
}

#[derive(Properties, PartialEq)]
pub struct EdcIdentityHubContextProviderProps {
  #[prop_or_default]
  pub children: Html,
  pub participant_id: String,
  pub participant_did: String,
}

#[component]
pub fn EdcIdentityHubContextProvider(props: &EdcIdentityHubContextProviderProps) -> Html {
  let latest_access_token_context = use_latest_access_token().unwrap();

  let edc_identity_hub_context = {
    let latest_access_token_context = latest_access_token_context.clone();

    use_state(move || EdcIdentityHubState {
      participant_id: props.participant_id.clone(),
      participant_did: props.participant_did.clone(),
      latest_access_token_context,
    })
  };

  use_effect_with(
    (
      props.participant_id.clone(),
      props.participant_did.clone(),
      edc_identity_hub_context.setter(),
      latest_access_token_context.clone(),
    ),
    |(
      participant_id,
      participant_did,
      edc_identity_hub_context_setter,
      latest_access_token_context,
    )| {
      edc_identity_hub_context_setter.set(EdcIdentityHubState {
        participant_id: participant_id.clone(),
        participant_did: participant_did.clone(),
        latest_access_token_context: latest_access_token_context.clone(),
      });
    },
  );

  html! {
    <ContextProvider<EdcIdentityHubState> context={(*edc_identity_hub_context).clone()}>
      { props.children.clone() }
    </ContextProvider<EdcIdentityHubState>>
  }
}

#[hook]
pub fn use_edc_identity_hub_context() -> EdcIdentityHubState {
  use_context::<EdcIdentityHubState>().expect("no EDC Identity Hub context found")
}
