use edc_connector_client::types::policy::Operator;
use patternfly_yew::prelude::*;
use yew::prelude::*;

static OPERATORS: [(&str, &str); 13] = [
  ("Equal", "eq"),
  ("Not equal", "neq"),
  ("Greater than", "gt"),
  ("Greater than or equal", "gteq"),
  ("Less than", "lt"),
  ("Less than or equal", "lteq"),
  ("Term less than or equal", "term-lteq"),
  ("Has part", "hasPart"),
  ("Is a", "isA"),
  ("Is all of", "isAllOf"),
  ("Is any of", "isAnyOf"),
  ("Is none of", "isNoneOf"),
  ("Is part of", "isPärtOf"),
];

#[derive(Clone, PartialEq, Properties)]
pub struct OperatorSelectorProps {
  pub operator: Operator,
  pub onchange: Callback<Operator>,
}

#[component]
pub fn OperatorSelector(props: &OperatorSelectorProps) -> Html {
  let operators = OPERATORS.iter().map(|(name, operator)| {
    let name = name.to_string();
    let operator = Operator::Simple(operator.to_string());
    let onchange = props.onchange.clone();

    html_nested!(
      <MenuAction selected=false onclick={onchange.reform(move |_| operator.clone())}>
        { name }
      </MenuAction>
    )
  });

  let label = match &props.operator {
    Operator::Simple(operator) => OPERATORS
      .iter()
      .find(|(_, operator_value)| *operator_value == operator)
      .map(|(operator_name, _)| operator_name.to_string())
      .unwrap_or(operator.to_string()),
    Operator::Id { id } => id.to_string(),
  };

  html!(<Dropdown text={label}>{ for operators }</Dropdown>)
}
