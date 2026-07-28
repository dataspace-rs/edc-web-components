use edc_connector_client::types::contract_negotiation::ContractNegotiationKind as EdcContractNegotiationKind;

#[derive(Debug, Clone, strum::Display)]
pub enum ContractNegotiationKind {
  Consumer,
  Provider,
}
impl From<&EdcContractNegotiationKind> for ContractNegotiationKind {
  fn from(kind: &EdcContractNegotiationKind) -> Self {
    match kind {
      EdcContractNegotiationKind::Consumer => ContractNegotiationKind::Consumer,
      EdcContractNegotiationKind::Provider => ContractNegotiationKind::Provider,
    }
  }
}
