use edc_identity_hub_client::{IdentityHubClient, IdentityHubClientVersion};
use std::{ops::Deref, rc::Rc};
use yew::prelude::*;
use yew_oauth2::context::LatestAccessToken;
use yew_oauth2::prelude::use_latest_access_token;

#[derive(Debug, PartialEq, Clone)]
pub enum EdcIdentityHubAction {}

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

impl Reducible for EdcIdentityHubState {
  type Action = EdcIdentityHubAction;

  fn reduce(self: Rc<Self>, _action: Self::Action) -> Rc<Self> {
    let new_self = self.deref().clone();

    new_self.into()
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

  let edc_connector_context = use_reducer(move || EdcIdentityHubState {
    participant_id: props.participant_id.clone(),
    participant_did: props.participant_did.clone(),
    latest_access_token_context,
  });

  html! {
    <ContextProvider<EdcIdentityHubContext> context={edc_connector_context}>
      { props.children.clone() }
    </ContextProvider<EdcIdentityHubContext>>
  }
}

pub type EdcIdentityHubContext = UseReducerHandle<EdcIdentityHubState>;

#[hook]
pub fn use_edc_identity_hub_context() -> EdcIdentityHubContext {
  use_context::<EdcIdentityHubContext>().expect("no EDC Identity Hub context found")
}
