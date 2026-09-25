use crate::components::{ConstraintRenderer, DidLabel};
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
    None
  } else {
    let permissions = props.permissions.iter().map(|permission| {
      html! {
        <ConstraintRenderer
          action={permission.action().clone()}
          constraints={permission.constraints().to_vec()}
        />
      }
    });

    Some(html_nested!(<DescriptionGroup term="Permissions">{ for permissions }</DescriptionGroup>))
  };

  let obligations = if props.obligations.is_empty() {
    None
  } else {
    let obligations = props.obligations.iter().map(|obligation| {
      html! {
        <ConstraintRenderer
          action={obligation.action().clone()}
          constraints={obligation.constraints().to_vec()}
        />
      }
    });

    Some(html_nested!(<DescriptionGroup term="Obligations">{ for obligations }</DescriptionGroup>))
  };

  let prohibitions = if props.prohibitions.is_empty() {
    None
  } else {
    let prohibitions = props.prohibitions.iter().map(|prohibition| {
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

  let extensible_properties = if props.extensible_properties.is_empty() {
    None
  } else {
    let extensible_properties = props.extensible_properties.iter().map(|(key, value)| {
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
      { permissions }
      { obligations }
      { prohibitions }
      { extensible_properties }
      { assigner }
      { assignee }
    </>
  )
}
