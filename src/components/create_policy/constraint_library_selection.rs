use super::list_of_rules::UpdatedRules;
use crate::components::ConstraintLibrarySearch;
use crate::components::ConstraintParameterInput;
use crate::components::create_policy::atomic_constraint_edit::ConstraintMode;
use crate::contexts::{ConstraintLibraryItem, RuleKind, use_constraint_library_context};
use edc_connector_client::types::policy::{Action, Constraint};
use patternfly_yew::prelude::*;
use serde_json::Value;
use yew::prelude::*;

/// Action used for the rule that library constraints are added to.
fn library_action() -> Action {
  Action::Simple("use".to_string())
}

/// The constraint created from `item` in the rules, if any.
fn find_constraint<'a>(
  rules: &'a UpdatedRules,
  item: &ConstraintLibraryItem,
) -> Option<&'a Constraint> {
  rules
    .iter()
    .flat_map(|(_, constraints)| constraints.iter())
    .map(|(_, constraint)| constraint)
    .find(|constraint| item.matches(constraint))
}

/// Adds the constraint to the first rule using the library action, creating that rule if needed.
fn add_constraint(mut rules: UpdatedRules, constraint: Constraint) -> UpdatedRules {
  let action = library_action();

  if let Some((_, constraints)) = rules.iter_mut().find(|(a, _)| *a == action) {
    constraints.push((ConstraintMode::Default, constraint));
  } else {
    rules.push((action, vec![(ConstraintMode::Default, constraint)]));
  }

  rules
}

/// Removes the constraints created from `item` from every rule, dropping rules that become
/// empty because of it.
fn remove_constraint(rules: UpdatedRules, item: &ConstraintLibraryItem) -> UpdatedRules {
  rules
    .into_iter()
    .filter_map(|(action, constraints)| {
      let before = constraints.len();
      let constraints: Vec<_> = constraints
        .into_iter()
        .filter(|(_, constraint)| !item.matches(constraint))
        .collect();

      if constraints.is_empty() && before > 0 {
        None
      } else {
        Some((action, constraints))
      }
    })
    .collect()
}

/// Sets the value of the parameter at `index` in the constraints created from `item`.
fn set_parameter_value(
  rules: UpdatedRules,
  item: &ConstraintLibraryItem,
  index: usize,
  value: Value,
) -> UpdatedRules {
  rules
    .into_iter()
    .map(|(action, constraints)| {
      let constraints = constraints
        .into_iter()
        .map(|(mode, constraint)| {
          if item.matches(&constraint) {
            let mut values = item.parameter_values(&constraint);
            if let Some(slot) = values.get_mut(index) {
              *slot = value.clone();
            }

            (mode, item.with_parameter_values(&values))
          } else {
            (mode, constraint)
          }
        })
        .collect();

      (action, constraints)
    })
    .collect()
}

#[derive(Clone, Debug, PartialEq)]
enum SelectionChange {
  Add,
  Remove,
  SetParameterValue(usize, Value),
}

#[derive(Clone, PartialEq, Properties)]
pub struct ConstraintLibrarySelectionProps {
  pub permissions: UpdatedRules,
  pub prohibitions: UpdatedRules,
  pub obligations: UpdatedRules,
  pub onchange: Callback<(RuleKind, UpdatedRules)>,
}

/// Simplified policy edition: pick constraints from the library by title and description.
#[component]
pub fn ConstraintLibrarySelection(props: &ConstraintLibrarySelectionProps) -> Html {
  let constraint_library_context = use_constraint_library_context();

  let ontoggle = use_callback(
    (
      props.permissions.clone(),
      props.prohibitions.clone(),
      props.obligations.clone(),
      props.onchange.clone(),
    ),
    |(item, change): (ConstraintLibraryItem, SelectionChange),
     (permissions, prohibitions, obligations, onchange)| {
      let rules = match item.rule_kind {
        RuleKind::Permission => permissions.clone(),
        RuleKind::Prohibition => prohibitions.clone(),
        RuleKind::Obligation => obligations.clone(),
      };

      let rules = match change {
        SelectionChange::Add if find_constraint(&rules, &item).is_some() => rules,
        SelectionChange::Add => add_constraint(rules, item.new_constraint()),
        SelectionChange::Remove => remove_constraint(rules, &item),
        SelectionChange::SetParameterValue(index, value) => {
          set_parameter_value(rules, &item, index, value)
        }
      };

      onchange.emit((item.rule_kind, rules));
    },
  );

  let onadd = use_callback(ontoggle.clone(), |item: ConstraintLibraryItem, ontoggle| {
    ontoggle.emit((item, SelectionChange::Add));
  });

  let items = constraint_library_context
    .map(|context| context.constraint_library().to_vec())
    .unwrap_or_default();

  if items.is_empty() {
    return html!(<EmptyState title="No constraint in the library" />);
  }

  let selected_constraint = |item: &ConstraintLibraryItem| {
    let rules = match item.rule_kind {
      RuleKind::Permission => &props.permissions,
      RuleKind::Prohibition => &props.prohibitions,
      RuleKind::Obligation => &props.obligations,
    };

    find_constraint(rules, item).cloned()
  };

  let choices = items
    .iter()
    .filter(|item| selected_constraint(item).is_none())
    .cloned()
    .collect::<Vec<_>>();

  let selected_items = items
    .iter()
    .filter_map(|item| selected_constraint(item).map(|constraint| (item, constraint)))
    .map(|(item, constraint)| {
      let item = item.clone();
      let name = item.name.clone();
      let description = item.description.clone().map(|description| {
        html!(
          <div
            style="
              font-size: var(--pf-t--global--font--size--sm);
              color: var(--pf-t--global--text--color--subtle)
            "
          >
            { description }
          </div>
        )
      });

      let parameters = item
        .parameters
        .iter()
        .cloned()
        .zip(item.parameter_values(&constraint))
        .enumerate()
        .map(|(index, (parameter, value))| {
          let item = item.clone();
          let onchange = ontoggle.reform(move |value: Value| {
            (
              item.clone(),
              SelectionChange::SetParameterValue(index, value),
            )
          });

          html_nested!(
            <SplitItem>
              <ConstraintParameterInput {parameter} {value} {onchange} />
            </SplitItem>
          )
        })
        .collect::<Vec<_>>();

      html_nested!(
        <StackItem>
          <Split gutter=true>
            <SplitItem fill=true>
              <strong>{ name }</strong>
              { for description }
            </SplitItem>
            { for parameters }
            <SplitItem>
              <Button
                variant={ButtonVariant::Danger}
                icon={Icon::Trash}
                aria_label="Remove constraint"
                onclick={ontoggle.reform(move |_| (item.clone(), SelectionChange::Remove))}
              />
            </SplitItem>
          </Split>
        </StackItem>
      )
    })
    .collect::<Vec<_>>();

  let selected_list = if selected_items.is_empty() {
    html!(
      <HelperText>
        <HelperTextItem>{ "No constraint selected" }</HelperTextItem>
      </HelperText>
    )
  } else {
    html!(
      <Card size={CardSize::Compact}>
        <CardBody>
          <Stack gutter=true>{ for selected_items }</Stack>
        </CardBody>
      </Card>
    )
  };

  html!(
    <Stack gutter=true>
      <StackItem>
        <ConstraintLibrarySearch items={choices} onselect={onadd} />
      </StackItem>
      <StackItem>{ selected_list }</StackItem>
    </Stack>
  )
}
