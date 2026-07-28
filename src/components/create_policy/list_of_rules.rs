use super::Rule;
use crate::components::create_policy::atomic_constraint_edit::ConstraintMode;
use edc_connector_client::types::policy::{Action, Constraint};
use patternfly_yew::prelude::*;
use yew::prelude::*;

pub type UpdatedRules = Vec<(Action, Vec<(ConstraintMode, Constraint)>)>;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub list: Vec<(Action, Vec<(ConstraintMode, Constraint)>)>,
  pub onchange: Callback<UpdatedRules>,
}

#[component]
pub fn ListOfRules(props: &Props) -> Html {
  let add_rule = use_callback(
    (props.list.clone(), props.onchange.clone()),
    |_, (list, onchange)| {
      let mut list_of_rules = (*list).clone();
      list_of_rules.push((Action::Simple("use".to_string()), vec![]));
      onchange.emit(list_of_rules);
    },
  );

  let onchange = use_callback(
    (props.list.clone(), props.onchange.clone()),
    |(index, action, constraints), (list, onchange)| {
      let mut list_of_rules = (*list).clone();
      list_of_rules[index] = (action, constraints);
      onchange.emit(list_of_rules);
    },
  );

  let ondelete = use_callback(
    (props.list.clone(), props.onchange.clone()),
    |index, (list, onchange)| {
      let mut list_of_rules = (*list).clone();
      list_of_rules.remove(index);
      onchange.emit(list_of_rules);
    },
  );

  log::info!("{:?}", props.list);

  let list_of_rules = props
    .list
    .iter()
    .enumerate()
    .map(|(index, (action, constraints))| {
      let action = action.clone();
      let constraints = constraints.clone();

      html_nested!(
        <StackItem>
          <Card>
            <CardTitle>{ format!("Rule {}", index + 1) }</CardTitle>
            <CardBody>
              <Rule
                key={index}
                {index}
                {action}
                {constraints}
                onchange={onchange.clone()}
                ondelete={ondelete.clone()}
              />
            </CardBody>
          </Card>
        </StackItem>
      )
    });

  html!(
    <Stack gutter=true>
      { for list_of_rules }
      <StackItem>
        <Button icon={Icon::Plus} variant={ButtonVariant::Primary} onclick={add_rule}>
          { "Add rule" }
        </Button>
      </StackItem>
    </Stack>
  )
}
