use edc_connector_client::{Auth, EdcConnectorClient};
use yew::prelude::*;
use yew_oauth2::context::LatestAccessToken;
use yew_oauth2::prelude::use_latest_access_token;

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

  let edc_connector_context = {
    let latest_access_token_context = latest_access_token_context.clone();

    use_state(move || EdcConnectorState {
      management_url: props.management_url.clone(),
      api_key: props.api_key.clone(),
      latest_access_token_context,
    })
  };

  use_effect_with(
    (
      props.management_url.clone(),
      props.api_key.clone(),
      edc_connector_context.setter(),
      latest_access_token_context.clone(),
    ),
    |(management_url, api_key, edc_connector_context_setter, latest_access_token_context)| {
      edc_connector_context_setter.set(EdcConnectorState {
        management_url: management_url.clone(),
        api_key: api_key.clone(),
        latest_access_token_context: latest_access_token_context.clone(),
      });
    },
  );

  html! {
    <ContextProvider<EdcConnectorState> context={(*edc_connector_context).clone()}>
      { props.children.clone() }
    </ContextProvider<EdcConnectorState>>
  }
}

#[hook]
pub fn use_edc_connector_context() -> EdcConnectorState {
  use_context::<EdcConnectorState>().expect("no EDC Connector context found")
}
