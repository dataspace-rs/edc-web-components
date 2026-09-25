use crate::components::ConstraintRenderer;
use crate::models::PolicyDefinitionItem;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PolicyCardProps {
  pub policy_definition_item: PolicyDefinitionItem,
  #[prop_or_default]
  pub selected: bool,
  #[prop_or_default]
  pub disabled: bool,
  #[prop_or("selectable-policy".to_string())]
  pub policy_selection_group_id: String,
  pub on_click: Callback<()>,
}

#[component]
pub fn PolicyCard(props: &PolicyCardProps) -> Html {
  let permissions = if props.policy_definition_item.permissions.is_empty() {
    None
  } else {
    let permissions = props
      .policy_definition_item
      .permissions
      .iter()
      .map(|permission| {
        html! {
          <ConstraintRenderer
            action={permission.action().clone()}
            constraints={permission.constraints().to_vec()}
          />
        }
      });

    Some(html_nested!(<DescriptionGroup term="Permissions">{ for permissions }</DescriptionGroup>))
  };

  let obligations = if props.policy_definition_item.obligations.is_empty() {
    None
  } else {
    let obligations = props
      .policy_definition_item
      .obligations
      .iter()
      .map(|obligation| {
        html! {
          <ConstraintRenderer
            action={obligation.action().clone()}
            constraints={obligation.constraints().to_vec()}
          />
        }
      });

    Some(html_nested!(<DescriptionGroup term="Obligations">{ for obligations }</DescriptionGroup>))
  };

  let prohibitions = if props.policy_definition_item.prohibitions.is_empty() {
    None
  } else {
    let prohibitions = props
      .policy_definition_item
      .prohibitions
      .iter()
      .map(|prohibition| {
        html! {
          <ConstraintRenderer
            action={prohibition.action().clone()}
            constraints={prohibition.constraints().to_vec()}
          />
        }
      });

    Some(
      html_nested!(<DescriptionGroup term="Prohibitions">{ for prohibitions }</DescriptionGroup>),
    )
  };

  let selectable_actions = yew::props!(CardSelectableActionsObjectProperties {
    action: CardSelectableActionsVariant::Click {
      onclick: Some(props.on_click.reform(move |_| ())),
    },
    base: yew::props!(CardSelectableActionsObjectBase {
      name: props.policy_selection_group_id.clone()
    })
  });

  let extensible_properties = if props
    .policy_definition_item
    .extensible_properties
    .is_empty()
  {
    None
  } else {
    let extensible_properties = props
      .policy_definition_item
      .extensible_properties
      .iter()
      .map(|(key, value)| {
        html! { <DescriptionGroup term={key.clone()}>{ value.to_string() }</DescriptionGroup> }
      });

    Some(html_nested!(
      <DescriptionGroup term="Extensible Properties">
        <DescriptionList mode={[DescriptionListMode::Horizontal]}>
          { for extensible_properties }
        </DescriptionList>
      </DescriptionGroup>
    ))
  };

  html!(
    <Card selectable=true disabled={props.disabled} selected={props.selected}>
      <CardHeader {selectable_actions}>
        <div>{ &props.policy_definition_item.name }</div>
      </CardHeader>
      <CardBody>
        <DescriptionList>
          { permissions }
          { obligations }
          { prohibitions }
          { extensible_properties }
        </DescriptionList>
      </CardBody>
    </Card>
  )
}
