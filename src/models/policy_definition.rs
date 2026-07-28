use crate::models::PolicyKind;
use edc_connector_client::types::policy::PolicyDefinition;

#[derive(Clone, Debug, PartialEq)]
pub struct PolicyDefinitionItem {
  pub id: String,
  pub kind: String,
  pub assignee: Option<String>,
  pub assigner: Option<String>,
}

impl From<PolicyDefinition> for PolicyDefinitionItem {
  fn from(policy_definition: PolicyDefinition) -> Self {
    PolicyDefinitionItem {
      id: policy_definition.id().to_string(),
      kind: PolicyKind::from(policy_definition.policy().kind()).to_string(),
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
