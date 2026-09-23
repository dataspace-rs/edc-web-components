use crate::contexts::{RedirectionAction, use_edc_connector_context, use_redirection_context};
use crate::models::PolicyDefinitionItem;
use edc_connector_client::EdcConnectorApiVersion;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PolicyReferenceProps {
  pub policy_id: String,
}

#[component]
pub fn PolicyReference(props: &PolicyReferenceProps) -> Html {
  html! {
    <Suspense fallback={html! {<Bullseye><Spinner size={SpinnerSize::Sm} /></Bullseye>}}>
      <PolicyReferenceInner policy_id={props.policy_id.clone()} />
    </Suspense>
  }
}

#[component]
pub fn PolicyReferenceInner(props: &PolicyReferenceProps) -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();
  let redirection_context = use_redirection_context();

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
    "Policy".to_string()
  };

  let onclick = redirection_context
    .map(|redirection_context| {
      let policy_id = props.policy_id.clone();

      redirection_context
        .redirect_to()
        .reform(move |_| RedirectionAction::Policy(policy_id.clone()))
    })
    .unwrap_or_default();

  Ok(html!(
    <Button variant={ButtonVariant::InlineLink} {onclick}>
      <Flex>
        <FlexItem modifiers={[FlexModifier::Align(Alignment::Center).all()]}>
          <yew_icons::Icon data={yew_icons::IconData::LUCIDE_SHIELD} />
        </FlexItem>
        <FlexItem modifiers={[FlexModifier::Align(Alignment::Center).all()]}>{ label }</FlexItem>
      </Flex>
    </Button>
  ))
}
