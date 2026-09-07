use crate::models::PolicyKind;
use edc_connector_client::types::policy::{
  Obligation, Permission, Policy, PolicyDefinition, Prohibition,
};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct PolicyDefinitionItem {
  pub id: String,
  pub name: String,
  pub kind: String,
  pub permissions: Vec<Permission>,
  pub obligations: Vec<Obligation>,
  pub prohibitions: Vec<Prohibition>,
  pub extensible_properties: HashMap<String, serde_json::Value>,
  pub assignee: Option<String>,
  pub assigner: Option<String>,
}

impl From<PolicyDefinition> for PolicyDefinitionItem {
  fn from(policy_definition: PolicyDefinition) -> Self {
    PolicyDefinitionItem {
      id: policy_definition.id().to_string(),
      name: policy_definition
        .private_property("name")
        .ok()
        .and_then(|name| name)
        .unwrap_or(policy_definition.id().to_string()),
      kind: PolicyKind::from(policy_definition.policy().kind()).to_string(),
      permissions: policy_definition.policy().permissions().to_vec(),
      obligations: policy_definition.policy().obligations().to_vec(),
      prohibitions: policy_definition.policy().prohibitions().to_vec(),
      extensible_properties: policy_definition.policy().extensible_properties().clone(),
      assignee: policy_definition
        .policy()
        .assignee()
        .map(|assignee| assignee.to_string()),
      assigner: policy_definition
        .policy()
        .assigner()
        .map(|assigner| assigner.to_string()),
    }
  }
}

impl From<&Policy> for PolicyDefinitionItem {
  fn from(policy: &Policy) -> Self {
    PolicyDefinitionItem {
      id: policy.id().cloned().unwrap_or_default(),
      name: policy.id().cloned().unwrap_or_default(),
      kind: PolicyKind::from(policy.kind()).to_string(),
      permissions: policy.permissions().to_vec(),
      obligations: policy.obligations().to_vec(),
      prohibitions: policy.prohibitions().to_vec(),
      extensible_properties: policy.extensible_properties().clone(),
      assignee: policy.assignee().cloned(),
      assigner: policy.assigner().cloned(),
    }
  }
}
