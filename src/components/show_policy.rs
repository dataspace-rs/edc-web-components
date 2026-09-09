use crate::components::ConstraintRenderer;
use edc_connector_client::types::policy::{Policy, PolicyKind};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ShowPolicyProps {
  pub policy: Policy,
  #[prop_or_default]
  pub name: Option<String>,
  #[prop_or_default]
  pub hide_kind: bool,
  #[prop_or_default]
  pub hide_id: bool,
  #[prop_or_default]
  pub hide_name: bool,
  #[prop_or_default]
  pub hide_profiles: bool,
  #[prop_or_default]
  pub hide_extensible_properties: bool,
}

#[component]
pub fn ShowPolicy(props: &ShowPolicyProps) -> Html {
  let permissions = props.policy.permissions().iter().map(|permission| {
    html! {
      <ConstraintRenderer
        action={permission.action().clone()}
        constraints={permission.constraints().to_vec()}
      />
    }
  });

  let obligations = props.policy.obligations().iter().map(|obligation| {
    html! {
      <ConstraintRenderer
        action={obligation.action().clone()}
        constraints={obligation.constraints().to_vec()}
      />
    }
  });

  let prohibitions = props.policy.prohibitions().iter().map(|prohibition| {
    html! {
      <ConstraintRenderer
        action={prohibition.action().clone()}
        constraints={prohibition.constraints().to_vec()}
      />
    }
  });

  let id = if props.hide_id {
    html!()
  } else {
    html!(<DescriptionGroup term="Id">{ props.policy.id() }</DescriptionGroup>)
  };

  let name = if props.hide_name {
    html!()
  } else {
    html!(<DescriptionGroup term="Name">{ props.name.clone() }</DescriptionGroup>)
  };

  let kind = if props.hide_kind {
    html!()
  } else {
    let kind = match props.policy.kind() {
      PolicyKind::Set => "Set",
      PolicyKind::Offer => "Offer",
      PolicyKind::Agreement => "Agreement",
    };

    html!(<DescriptionGroup term="Kind">{ kind }</DescriptionGroup>)
  };

  let profiles = if props.hide_profiles {
    html!()
  } else {
    let profiles = props.policy.profiles().iter().map(|profile| {
      html_nested! {
      <FlexItem>
        <Label color={Color::Blue} label={profile.to_string()} />
      </FlexItem>
    }
    });

    html!(
      <DescriptionGroup term="Profiles">
        <Flex>{ for profiles }</Flex>
      </DescriptionGroup>
    )
  };

  let extensible_properties = if props.hide_extensible_properties {
    html!()
  } else {
    let extensible_properties = props
      .policy
      .extensible_properties()
      .iter()
      .map(|(key, value)| {
        html_nested! {
        <StackItem>
          <DescriptionGroup term={key.to_string()}>
            <CodeBlock>
              <CodeBlockCode>
                { serde_json::to_string_pretty(value).unwrap_or_default() }
              </CodeBlockCode>
            </CodeBlock>
          </DescriptionGroup>
        </StackItem>
      }
      });

    html!(
      <DescriptionGroup term="Extensible Properties">
        <DescriptionList mode={[DescriptionListMode::Horizontal]}>
          { for extensible_properties }
        </DescriptionList>
      </DescriptionGroup>
    )
  };

  html!(
    <DescriptionList mode={[DescriptionListMode::Horizontal]}>
      { id }
      { name }
      { kind }
      <DescriptionGroup term="Assigner">
        { props.policy.assigner().cloned().unwrap_or_default() }
      </DescriptionGroup>
      <DescriptionGroup term="Assignee">
        { props.policy.assignee().cloned().unwrap_or_default() }
      </DescriptionGroup>
      <DescriptionGroup term="Permissions">{ for permissions }</DescriptionGroup>
      <DescriptionGroup term="Obligations">{ for obligations }</DescriptionGroup>
      <DescriptionGroup term="Prohibitions">{ for prohibitions }</DescriptionGroup>
      { profiles }
      { extensible_properties }
    </DescriptionList>
  )
}
