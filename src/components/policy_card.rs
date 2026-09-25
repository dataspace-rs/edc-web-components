use crate::components::PolicyProperties;
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
  let selectable_actions = yew::props!(CardSelectableActionsObjectProperties {
    action: CardSelectableActionsVariant::Click {
      onclick: Some(props.on_click.reform(move |_| ())),
    },
    base: yew::props!(CardSelectableActionsObjectBase {
      name: props.policy_selection_group_id.clone()
    })
  });

  html!(
    <Card selectable=true disabled={props.disabled} selected={props.selected}>
      <CardHeader {selectable_actions}>
        <div>{ &props.policy_definition_item.name }</div>
      </CardHeader>
      <CardBody>
        <DescriptionList>
          <PolicyProperties
            permissions={props.policy_definition_item.permissions.clone()}
            obligations={props.policy_definition_item.obligations.clone()}
            prohibitions={props.policy_definition_item.prohibitions.clone()}
            extensible_properties={props.policy_definition_item.extensible_properties.clone()}
            assigner={props.policy_definition_item.assigner.clone()}
            assignee={props.policy_definition_item.assignee.clone()}
          />
        </DescriptionList>
      </CardBody>
    </Card>
  )
}
