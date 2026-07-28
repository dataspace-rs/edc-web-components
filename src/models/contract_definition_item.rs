use edc_connector_client::types::contract_definition::ContractDefinition;

#[derive(Clone, Debug, PartialEq)]
pub struct ContractDefinitionItem {
  pub id: String,
  pub access_policy_id: String,
  pub contract_policy_id: String,
  pub asset_ids: Vec<String>,
}

impl From<ContractDefinition> for ContractDefinitionItem {
  fn from(contract_definition: ContractDefinition) -> Self {
    let id = contract_definition.id().to_string();
    let access_policy_id = contract_definition.access_policy_id().to_string();
    let contract_policy_id = contract_definition.contract_policy_id().to_string();
    let asset_ids = contract_definition
      .assets_selector()
      .iter()
      .map(|criterion| match &criterion.operand_right().0 {
        serde_json::Value::String(s) => s.to_string(),
        _ => String::from("Invalid asset id."),
      })
      .collect::<Vec<_>>();

    Self {
      id,
      access_policy_id,
      contract_policy_id,
      asset_ids,
    }
  }
}
