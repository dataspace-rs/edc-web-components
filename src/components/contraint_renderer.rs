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

  let action = match &props.action {
    Action::Simple(simple) => simple.to_string(),
    Action::Id { id } => id.to_string(),
  };

  let action = action.replace("http://www.w3.org/ns/odrl/2/", "");

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

      let operator = match operator.as_str() {
        "http://www.w3.org/ns/odrl/2/eq" => "Equal".to_string(),
        "http://www.w3.org/ns/odrl/2/neq" => "Not equal".to_string(),
        "http://www.w3.org/ns/odrl/2/gt" => "Greater than".to_string(),
        "http://www.w3.org/ns/odrl/2/gteq" => "Greater than or equal".to_string(),
        "http://www.w3.org/ns/odrl/2/lt" => "Less than".to_string(),
        "http://www.w3.org/ns/odrl/2/lteq" => "Less than or equal".to_string(),
        "http://www.w3.org/ns/odrl/2/term-lteq" => "Term less than or equal".to_string(),
        "http://www.w3.org/ns/odrl/2/hasPart" => "Has part".to_string(),
        "http://www.w3.org/ns/odrl/2/isA" => "Is a".to_string(),
        "http://www.w3.org/ns/odrl/2/isAllOf" => "Is all of".to_string(),
        "http://www.w3.org/ns/odrl/2/isAnyOf" => "Is any of".to_string(),
        "http://www.w3.org/ns/odrl/2/isNoneOf" => "Is none of".to_string(),
        "http://www.w3.org/ns/odrl/2/isPärtOf" => "Is part of".to_string(),
        _ => operator,
      };

      let right_operand = match &atomic_constraint.right_operand.0 {
        serde_json::Value::String(content) => content.to_string(),
        value => value.to_string(),
      };

      html_nested!(
        <DescriptionGroup term="Constraint">
          <Flex>
            <FlexItem>
              <Label label={left_operand} color={Color::Blue} />
            </FlexItem>
            <FlexItem>
              <Label label={operator} color={Color::Blue} />
            </FlexItem>
            <FlexItem>
              <Label label={right_operand} color={Color::Blue} />
            </FlexItem>
          </Flex>
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
