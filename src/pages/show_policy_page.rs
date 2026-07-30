use crate::components::ConstraintRenderer;
use crate::contexts::use_edc_connector_context;
use edc_connector_client::types::policy::PolicyKind;
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
    let kind = match policy.policy().kind() {
      PolicyKind::Set => "Set",
      PolicyKind::Offer => "Offer",
      PolicyKind::Agreement => "Agreement",
    };

    let permissions = policy.policy().permissions().iter().map(|permission| {
      html! {
        <ConstraintRenderer
          action={permission.action().clone()}
          constraints={permission.constraints().to_vec()}
        />
      }
    });

    let obligations = policy.policy().obligations().iter().map(|obligation| {
      html! {
        <ConstraintRenderer
          action={obligation.action().clone()}
          constraints={obligation.constraints().to_vec()}
        />
      }
    });

    let prohibitions = policy.policy().prohibitions().iter().map(|prohibition| {
      html! {
        <ConstraintRenderer
          action={prohibition.action().clone()}
          constraints={prohibition.constraints().to_vec()}
        />
      }
    });

    Ok(html!(
      <DescriptionList mode={[DescriptionListMode::Horizontal]}>
        <DescriptionGroup term="Id">{ policy.id() }</DescriptionGroup>
        <DescriptionGroup term="Kind">{ kind }</DescriptionGroup>
        <DescriptionGroup term="Assigner">
          { policy.policy().assigner().cloned().unwrap_or_default() }
        </DescriptionGroup>
        <DescriptionGroup term="Assignee">
          { policy.policy().assignee().cloned().unwrap_or_default() }
        </DescriptionGroup>
        <DescriptionGroup term="Permissions">{ for permissions }</DescriptionGroup>
        <DescriptionGroup term="Obligations">{ for obligations }</DescriptionGroup>
        <DescriptionGroup term="Prohibitions">{ for prohibitions }</DescriptionGroup>
      </DescriptionList>
    ))
  } else {
    Ok(html!(
      format!(
      "Policy with id {} not found.",
      props.policy_id
    )
    ))
  }
}
