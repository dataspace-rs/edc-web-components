use crate::contexts::use_edc_connector_context;
use crate::models::PolicyDefinitionItem;
use edc_connector_client::EdcConnectorApiVersion;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PolicyReferenceProps {
  pub policy_id: String,
  pub on_click: Callback<()>,
}

#[component]
pub fn PolicyReference(props: &PolicyReferenceProps) -> Html {
  html! {
    <Suspense fallback={html! {<Bullseye><Spinner size={SpinnerSize::Sm} /></Bullseye>}}>
      <PolicyReferenceInner policy_id={props.policy_id.clone()} on_click={props.on_click.clone()} />
    </Suspense>
  }
}

#[component]
pub fn PolicyReferenceInner(props: &PolicyReferenceProps) -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();

  let policy = use_future_with(
    (props.policy_id.clone(), edc_connector_context.clone()),
    async move |parameters| {
      let (policy_id, edc_connector_context) = (*parameters).clone();

      if let Some(edc_connector_client) = edc_connector_context.get_client() {
        edc_connector_client
          .policies(EdcConnectorApiVersion::V4)
          .get(&policy_id)
          .await
          .ok()
          .map(PolicyDefinitionItem::from)
      } else {
        None
      }
    },
  )?;

  let policy = (*policy).clone();

  let label = if let Some(policy) = policy {
    policy.name
  } else {
    props.policy_id.clone()
  };

  Ok(html!(
    <Button variant={ButtonVariant::InlineLink} onclick={props.on_click.reform(|_| ())}>
      { label }
    </Button>
  ))
}
