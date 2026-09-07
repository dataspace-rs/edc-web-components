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
  pub on_click: Callback<()>,
}

#[component]
pub fn PolicyCard(props: &PolicyCardProps) -> Html {
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

  let selectable_actions = {
    let name = props.policy_definition_item.id.clone();

    yew::props!(CardSelectableActionsObjectProperties {
      action: CardSelectableActionsVariant::SingleSelect {
        onchange: Some(props.on_click.reform(move |_| ())),
      },
      base: yew::props!(CardSelectableActionsObjectBase { name })
    })
  };

  let extensible_properties = props
    .policy_definition_item
    .extensible_properties
    .iter()
    .map(|(key, value)| {
      html! { <DescriptionGroup term={key.clone()}>{ value.to_string() }</DescriptionGroup> }
    });

  html!(
    <Card selectable=true disabled={props.disabled} selected={props.selected}>
      <CardHeader {selectable_actions}>
        <div>{ &props.policy_definition_item.name }</div>
      </CardHeader>
      <CardBody>
        <DescriptionList>
          <DescriptionGroup term="Permissions">{ for permissions }</DescriptionGroup>
          <DescriptionGroup term="Obligations">{ for obligations }</DescriptionGroup>
          <DescriptionGroup term="Prohibitions">{ for prohibitions }</DescriptionGroup>
          <DescriptionGroup term="Extensible Properties">
            <DescriptionList mode={[DescriptionListMode::Horizontal]}>
              { for extensible_properties }
            </DescriptionList>
          </DescriptionGroup>
        </DescriptionList>
      </CardBody>
    </Card>
  )
}
