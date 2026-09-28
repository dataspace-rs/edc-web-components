use crate::components::{ConstraintRenderer, DidLabel, Identifier};
use edc_connector_client::types::policy::{Obligation, Permission, Prohibition};
use patternfly_yew::prelude::*;
use std::collections::HashMap;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PolicyPropertiesProps {
  pub permissions: Vec<Permission>,
  pub obligations: Vec<Obligation>,
  pub prohibitions: Vec<Prohibition>,
  pub extensible_properties: HashMap<String, serde_json::Value>,
  pub assigner: Option<String>,
  pub assignee: Option<String>,
}

#[component]
pub fn PolicyProperties(props: &PolicyPropertiesProps) -> Html {
  let permissions = if props.permissions.is_empty() {
    html!(
      <HelperText>
        <HelperTextItem variant={HelperTextItemVariant::Intermediate}>{ "None" }</HelperTextItem>
      </HelperText>
    )
  } else {
    props
      .permissions
      .iter()
      .map(|permission| {
        html! {
          <ConstraintRenderer
            action={permission.action().clone()}
            constraints={permission.constraints().to_vec()}
          />
        }
      })
      .collect::<Html>()
  };

  let obligations = if props.obligations.is_empty() {
    html!(
      <HelperText>
        <HelperTextItem variant={HelperTextItemVariant::Intermediate}>{ "None" }</HelperTextItem>
      </HelperText>
    )
  } else {
    props
      .obligations
      .iter()
      .map(|obligation| {
        html! {
          <ConstraintRenderer
            action={obligation.action().clone()}
            constraints={obligation.constraints().to_vec()}
          />
        }
      })
      .collect::<Html>()
  };

  let prohibitions = if props.prohibitions.is_empty() {
    html!(
      <HelperText>
        <HelperTextItem variant={HelperTextItemVariant::Intermediate}>{ "None" }</HelperTextItem>
      </HelperText>
    )
  } else {
    props
      .prohibitions
      .iter()
      .map(|prohibition| {
        html! {
          <ConstraintRenderer
            action={prohibition.action().clone()}
            constraints={prohibition.constraints().to_vec()}
          />
        }
      })
      .collect::<Html>()
  };

  let extensible_properties = if props.extensible_properties.is_empty() {
    None
  } else {
    let extensible_properties =
      props
        .extensible_properties
        .iter()
        .enumerate()
        .map(|(index, (key, value))| {
          html_nested! {
            <SimpleListItem key={index}>
              <Identifier id={key.clone()} />
              <div>{ value.to_string() }</div>
            </SimpleListItem>
          }
        });

    Some(html_nested!(
      <DescriptionGroup term="Extensible Properties">
        <SimpleList>{ for extensible_properties }</SimpleList>
      </DescriptionGroup>
    ))
  };

  let assigner = props
    .assigner
    .as_ref()
    .map(|assigner| {
      Some(html_nested! {
        <StackItem>
          <DescriptionGroup term="Assigner">
            <DidLabel did={assigner.clone()} />
          </DescriptionGroup>
        </StackItem>
      })
    })
    .unwrap_or_default();

  let assignee = props
    .assignee
    .as_ref()
    .map(|assignee| {
      Some(html_nested! {
        <StackItem>
          <DescriptionGroup term="Assignee">
            <DidLabel did={assignee.clone()} />
          </DescriptionGroup>
        </StackItem>
      })
    })
    .unwrap_or_default();

  html!(
    <>
      <DescriptionGroup term="Permissions">{ permissions }</DescriptionGroup>
      <DescriptionGroup term="Obligations">{ obligations }</DescriptionGroup>
      <DescriptionGroup term="Prohibitions">{ prohibitions }</DescriptionGroup>
      { extensible_properties }
      { assigner }
      { assignee }
    </>
  )
}
