use edc_connector_client::types::contract_agreement::ContractAgreement;

#[derive(Clone, Debug, PartialEq)]
pub struct ContractAgreementItem {
  pub id: String,
  pub signing_date: String,
  pub consumer_id: String,
  pub provider_id: String,
  pub asset_id: String,
  pub policy_id: String,
}

impl From<ContractAgreement> for ContractAgreementItem {
  fn from(contract_agreement: ContractAgreement) -> Self {
    let id = contract_agreement.id().to_string();
    let signing_date =
      chrono::DateTime::from_timestamp(contract_agreement.contract_signing_date(), 0)
        .unwrap_or_default()
        .to_string();

    let consumer_id = contract_agreement.consumer_id().to_string();
    let provider_id = contract_agreement.provider_id().to_string();
    let asset_id = contract_agreement.asset_id().to_string();
    let policy_id = contract_agreement
      .policy()
      .id()
      .map(|id| id.to_string())
      .unwrap_or_default();

    Self {
      id,
      signing_date,
      consumer_id,
      provider_id,
      asset_id,
      policy_id,
    }
  }
}
