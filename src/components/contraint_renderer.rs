use crate::contexts::use_policy_constraint_renderer_context;
use edc_connector_client::types::policy::{Constraint, LeftOperand, Operator};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ConstraintRendererProps {
  pub action: edc_connector_client::types::policy::Action,
  pub constraints: Vec<Constraint>,
}

#[component]
pub fn ConstraintRenderer(props: &ConstraintRendererProps) -> Html {
  use edc_connector_client::types::policy::Action;

  let policy_constraint_renderer_context = use_policy_constraint_renderer_context();

  let action = match &props.action {
    Action::Simple(simple) => simple.to_string(),
    Action::Id { id } => id.to_string(),
  };

  let constraints = props.constraints.iter().map(|constraint| {
    let (title, rendered_context) = if let Some((title, rendered_context)) =
      policy_constraint_renderer_context
        .as_ref()
        .and_then(|policy_constraint_renderer_context| {
          policy_constraint_renderer_context.renderer_constraint(&action, constraint)
        }) {
      (title, rendered_context)
    } else {
      match constraint {
        Constraint::Atomic(atomic_constraint) => {
          let left_operand = match &atomic_constraint.left_operand {
            LeftOperand::Id { id } => id.to_string(),
            LeftOperand::Simple(simple) => simple.to_string(),
          };

          let operator = match &atomic_constraint.operator {
            Operator::Id { id } => id.to_string(),
            Operator::Simple(simple) => simple.to_string(),
          };

          let operator = match operator
            .replace("http://www.w3.org/ns/odrl/2/", "")
            .as_str()
          {
            "eq" => "Equal".to_string(),
            "neq" => "Not equal".to_string(),
            "gt" => "Greater than".to_string(),
            "gteq" => "Greater than or equal".to_string(),
            "lt" => "Less than".to_string(),
            "lteq" => "Less than or equal".to_string(),
            "term-lteq" => "Term less than or equal".to_string(),
            "hasPart" => "Has part".to_string(),
            "isA" => "Is a".to_string(),
            "isAllOf" => "Is all of".to_string(),
            "isAnyOf" => "Is any of".to_string(),
            "isNoneOf" => "Is none of".to_string(),
            "isPärtOf" => "Is part of".to_string(),
            _ => operator,
          };

          let right_operand = match &atomic_constraint.right_operand.0 {
            serde_json::Value::String(content) => content.to_string(),
            value => value.to_string(),
          };

          let action = action.replace("http://www.w3.org/ns/odrl/2/", "");

          let inner = html!(
            <Flex>
              <FlexItem>{ left_operand }</FlexItem>
              <FlexItem>
                <Label label={operator} color={Color::Blue} />
              </FlexItem>
              <FlexItem>{ right_operand }</FlexItem>
            </Flex>
          );

          (format!("{action} Constraint"), inner)
        }
        Constraint::MultiplicityConstraint(multiplicity_constraint) => (
          format!("{action} Multiplicity Constraint"),
          html! { format!("{multiplicity_constraint:?}") },
        ),
      }
    };

    html_nested!(
      <StackItem>
        <Alert {title} r#type={AlertType::Info}>{ rendered_context }</Alert>
      </StackItem>
    )
  });

  html! { <Stack gutter=true>{ for constraints }</Stack> }
}
