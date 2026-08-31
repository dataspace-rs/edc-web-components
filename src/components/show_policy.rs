use crate::components::ConstraintRenderer;
use edc_connector_client::types::policy::{Policy, PolicyKind};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ShowPolicyProps {
  pub policy: Policy,
}

#[component]
pub fn ShowPolicy(props: &ShowPolicyProps) -> Html {
  let kind = match props.policy.kind() {
    PolicyKind::Set => "Set",
    PolicyKind::Offer => "Offer",
    PolicyKind::Agreement => "Agreement",
  };

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

  let profiles = props.policy.profiles().iter().map(|profile| {
    html_nested! {
      <FlexItem>
        <Label color={Color::Blue} label={profile.to_string()} />
      </FlexItem>
    }
  });

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
    <DescriptionList mode={[DescriptionListMode::Horizontal]}>
      <DescriptionGroup term="Id">{ props.policy.id() }</DescriptionGroup>
      <DescriptionGroup term="Kind">{ kind }</DescriptionGroup>
      <DescriptionGroup term="Assigner">
        { props.policy.assigner().cloned().unwrap_or_default() }
      </DescriptionGroup>
      <DescriptionGroup term="Assignee">
        { props.policy.assignee().cloned().unwrap_or_default() }
      </DescriptionGroup>
      <DescriptionGroup term="Permissions">{ for permissions }</DescriptionGroup>
      <DescriptionGroup term="Obligations">{ for obligations }</DescriptionGroup>
      <DescriptionGroup term="Prohibitions">{ for prohibitions }</DescriptionGroup>
      <DescriptionGroup term="Profiles">
        <Flex>{ for profiles }</Flex>
      </DescriptionGroup>
      <DescriptionGroup term="Extensible Properties">
        <DescriptionList mode={[DescriptionListMode::Horizontal]}>
          { for extensible_properties }
        </DescriptionList>
      </DescriptionGroup>
    </DescriptionList>
  )
}
