mod policy_properties;

use edc_connector_client::types::policy::{Policy, PolicyKind};
use patternfly_yew::prelude::*;
pub use policy_properties::PolicyProperties;
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

  html!(
    <DescriptionList mode={[DescriptionListMode::Horizontal]}>
      { id }
      { name }
      { kind }
      <PolicyProperties
        permissions={props.policy.permissions().to_vec()}
        obligations={props.policy.obligations().to_vec()}
        prohibitions={props.policy.prohibitions().to_vec()}
        extensible_properties={props.policy.extensible_properties().clone()}
        assigner={props.policy.assigner().cloned()}
        assignee={props.policy.assignee().cloned()}
      />
    </DescriptionList>
  )
}
