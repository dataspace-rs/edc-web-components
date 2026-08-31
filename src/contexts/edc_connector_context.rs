use edc_connector_client::{Auth, EdcConnectorClient};
use std::{ops::Deref, rc::Rc};
use yew::prelude::*;
use yew_oauth2::context::LatestAccessToken;
use yew_oauth2::prelude::use_latest_access_token;

#[derive(Debug, PartialEq, Clone)]
pub enum EdcConnectorAction {}

#[derive(Clone, PartialEq)]
pub struct EdcConnectorState {
  management_url: String,
  api_key: Option<String>,
  latest_access_token_context: Option<LatestAccessToken>,
}

impl EdcConnectorState {
  pub fn get_client(&self) -> Option<EdcConnectorClient> {
    let builder = EdcConnectorClient::builder().management_url(self.management_url.clone());

    let builder = if let Some(api_key) = self.api_key.as_ref() {
      builder.with_auth(Auth::ApiToken(api_key.clone()))
    } else {
      builder
    };

    let builder = if let Some(access_token) = self
      .latest_access_token_context
      .as_ref()
      .and_then(|latest_access_token_context| latest_access_token_context.access_token())
    {
      builder.with_auth(Auth::BearerToken(access_token))
    } else {
      builder
    };

    builder.build().ok()
  }
}

impl Reducible for EdcConnectorState {
  type Action = EdcConnectorAction;

  fn reduce(self: Rc<Self>, _action: Self::Action) -> Rc<Self> {
    let new_self = self.deref().clone();

    new_self.into()
  }
}

#[derive(Properties, PartialEq)]
pub struct EdcConnectorContextProviderProps {
  #[prop_or_default]
  pub children: Html,
  pub management_url: String,
  pub api_key: Option<String>,
}

#[component]
pub fn EdcConnectorContextProvider(props: &EdcConnectorContextProviderProps) -> Html {
  let latest_access_token_context = use_latest_access_token();

  let edc_connector_context = use_reducer(move || EdcConnectorState {
    management_url: props.management_url.clone(),
    api_key: props.api_key.clone(),
    latest_access_token_context,
  });

  html! {
    <ContextProvider<EdcConnectorContext> context={edc_connector_context}>
      { props.children.clone() }
    </ContextProvider<EdcConnectorContext>>
  }
}

pub type EdcConnectorContext = UseReducerHandle<EdcConnectorState>;

#[hook]
pub fn use_edc_connector_context() -> EdcConnectorContext {
  use_context::<EdcConnectorContext>().expect("no EDC Connector context found")
}
