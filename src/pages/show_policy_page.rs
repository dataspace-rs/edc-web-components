use crate::components::ShowPolicy;
use crate::contexts::use_edc_connector_context;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ShowPolicyPageProps {
  pub policy_id: String,
}

#[component]
pub fn ShowPolicyPage(props: &ShowPolicyPageProps) -> Html {
  html!(
    <>
      <Title level={Level::H2} size={Size::XXXLarge}>{ "Policy" }</Title>
      <Suspense fallback="Loading ...">
        <ShowPolicyPageInner policy_id={props.policy_id.clone()} />
      </Suspense>
    </>
  )
}

#[component]
pub fn ShowPolicyPageInner(props: &ShowPolicyPageProps) -> HtmlResult {
  let edc_connector_client = use_edc_connector_context();

  let policy = use_future_with(
    (props.policy_id.clone(), edc_connector_client.clone()),
    |properties| async move {
      let (policy_id, edc_connector_client) = (*properties).clone();

      if let Some(client) = edc_connector_client.get_client() {
        client
          .policies(edc_connector_client::EdcConnectorApiVersion::V4)
          .get(&policy_id)
          .await
          .ok()
      } else {
        None
      }
    },
  )?;

  let policy = (*policy).clone();

  if let Some(policy) = policy {
    Ok(html!(<ShowPolicy policy={policy.policy().clone()} />))
  } else {
    Ok(html!(format!(
      "Policy with id {} not found.",
      props.policy_id
    )))
  }
}
