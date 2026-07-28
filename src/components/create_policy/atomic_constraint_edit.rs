mod cel_atomic_constraint_edit;
mod default_atomic_constraint_edit;

use cel_atomic_constraint_edit::CelAtomicConstraintEdit;
use default_atomic_constraint_edit::DefaultAtomicConstraintEdit;
use edc_connector_client::types::policy::{LeftOperand, Operator};
use patternfly_yew::prelude::*;
use serde_json::Value;
use yew::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConstraintMode {
  Default,
  Cel,
}

#[derive(Clone, PartialEq, Properties)]
pub struct AtomicConstraintEditProps {
  pub index: usize,
  pub constraint_mode: ConstraintMode,
  pub left_operand: LeftOperand,
  pub operator: Operator,
  pub right_operand: Value,
  pub onchange: Callback<(usize, ConstraintMode, LeftOperand, Operator, Value)>,
  pub ondelete: Callback<usize>,
}

#[component]
pub fn AtomicConstraintEdit(props: &AtomicConstraintEditProps) -> Html {
  let onchange = use_callback(
    (
      props.index,
      props.left_operand.clone(),
      props.operator.clone(),
      props.right_operand.clone(),
      props.onchange.clone(),
    ),
    |constraint_mode: ConstraintMode, (index, left_operand, operator, right_operand, onchange)| {
      onchange.emit((
        *index,
        constraint_mode,
        left_operand.clone(),
        operator.clone(),
        right_operand.clone(),
      ));
    },
  );

  let label = match props.constraint_mode {
    ConstraintMode::Default => "Default",
    ConstraintMode::Cel => "CEL",
  };

  let on_change_cel = use_callback(
    (props.index, props.onchange.clone()),
    |cel_left_operand, (index, onchange)| {
      onchange.emit((
        *index,
        ConstraintMode::Cel,
        LeftOperand::Simple(cel_left_operand),
        Operator::Simple("eq".to_string()),
        Value::String("active".to_string()),
      ));
    },
  );

  let inner = match props.constraint_mode {
    ConstraintMode::Default => {
      html!(
        <DefaultAtomicConstraintEdit
          index={props.index}
          left_operand={props.left_operand.clone()}
          operator={props.operator.clone()}
          right_operand={props.right_operand.clone()}
          onchange={props.onchange.reform(|(index, left_operand, operator, right_operand)| {
            (index, ConstraintMode::Default, left_operand, operator, right_operand)
          })}
        />
      )
    }
    ConstraintMode::Cel => {
      let cel_left_operand = match props.left_operand.clone() {
        LeftOperand::Simple(value) => value,
        LeftOperand::Id { id } => id,
      };

      html!(<CelAtomicConstraintEdit {cel_left_operand} on_change={on_change_cel} />)
    }
  };

  let delete_constraint = use_callback(
    (props.index, props.ondelete.clone()),
    |_, (index, ondelete)| {
      ondelete.emit(*index);
    },
  );

  html!(
    <Flex>
      <FlexItem>
        <Dropdown text={label}>
          <MenuAction
            selected={props.constraint_mode == ConstraintMode::Cel}
            onclick={onchange.reform(move |_| ConstraintMode::Cel)}
          >
            { "CEL" }
          </MenuAction>
          <MenuAction
            selected={props.constraint_mode == ConstraintMode::Default}
            onclick={onchange.reform(move |_| ConstraintMode::Default)}
          >
            { "Default" }
          </MenuAction>
        </Dropdown>
      </FlexItem>
      <FlexItem modifiers={[FlexModifier::Flex1]}>{ inner }</FlexItem>
      <FlexItem>
        <Button
          icon={Icon::Trash}
          variant={ButtonVariant::DangerSecondary}
          onclick={delete_constraint}
        />
      </FlexItem>
    </Flex>
  )
}
