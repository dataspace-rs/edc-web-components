use super::{ContractNegotiationKind, ContractNegotiationState};
use edc_connector_client::types::contract_negotiation::ContractNegotiation;

#[derive(Clone, Debug, PartialEq)]
pub struct ContractNegotiationItem {
  pub id: String,
  pub state: String,
  pub contract_agreement_id: String,
  pub counter_party_id: String,
  pub counter_party_address: String,
  pub protocol: String,
  pub kind: String,
}

impl From<ContractNegotiation> for ContractNegotiationItem {
  fn from(contract_negotiation: ContractNegotiation) -> Self {
    let id = contract_negotiation.id().to_string();
    let state = ContractNegotiationState::from(contract_negotiation.state()).to_string();

    let contract_agreement_id = contract_negotiation
      .contract_agreement_id()
      .map(|contract_agreement_id| contract_agreement_id.to_string())
      .unwrap_or_default();

    let counter_party_id = contract_negotiation
      .counter_party_id()
      .clone()
      .unwrap_or_default();

    let counter_party_address = contract_negotiation.counter_party_address().to_string();
    let protocol = contract_negotiation.protocol().to_string();
    let kind = ContractNegotiationKind::from(contract_negotiation.kind()).to_string();

    Self {
      id,
      state,
      contract_agreement_id,
      counter_party_id,
      counter_party_address,
      protocol,
      kind,
    }
  }
}
