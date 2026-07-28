use crate::contexts::use_edc_connector_context;
use edc_connector_client::types::policy::{Constraint, LeftOperand, Operator, PolicyKind};
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
      <Title level={Level::H2} size={Size::XXXLarge}>{ "Show Policy" }</Title>
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

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ConstraintRendererProps {
  pub action: edc_connector_client::types::policy::Action,
  pub constraints: Vec<Constraint>,
}

#[component]
fn ConstraintRenderer(props: &ConstraintRendererProps) -> Html {
  use edc_connector_client::types::policy::Action;

  let action = match &props.action {
    Action::Simple(simple) => simple.to_string(),
    Action::Id { id } => id.to_string(),
  };

  let constraints = props.constraints.iter().map(|constraint| match constraint {
    Constraint::Atomic(atomic_constraint) => {
      let left_operand = match &atomic_constraint.left_operand {
        LeftOperand::Id { id } => id.to_string(),
        LeftOperand::Simple(simple) => simple.to_string(),
      };

      let operator = match &atomic_constraint.operator {
        Operator::Id { id } => id.to_string(),
        Operator::Simple(simple) => simple.to_string(),
      };

      let right_operand = match &atomic_constraint.right_operand.0 {
        serde_json::Value::String(content) => content.to_string(),
        value => value.to_string(),
      };

      html_nested!(
        <DescriptionGroup term="Contraint">
          { format!("{left_operand} {operator} {right_operand}") }
        </DescriptionGroup>
      )
    }
    Constraint::MultiplicityConstraint(multiplicity_constraint) => {
      html_nested!(
        <DescriptionGroup term="Multiplicity Constraint">
          { format!("{multiplicity_constraint:?}") }
        </DescriptionGroup>
      )
    }
  });

  html! {
    <>
      <DescriptionList>
        <DescriptionGroup term="Action">{ action }</DescriptionGroup>
        { for constraints }
      </DescriptionList>
    </>
  }
}
