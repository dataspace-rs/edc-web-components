use edc_connector_client::types::contract_negotiation::ContractNegotiationState as EdcContractNegotiationState;

#[derive(Debug, Clone, strum::Display)]
pub enum ContractNegotiationState {
  Initial,
  Requesting,
  Requested,
  Offering,
  Offered,
  Accepting,
  Accepted,
  Agreeing,
  Agreed,
  Verifying,
  Verified,
  Finalizing,
  Finalized,
  Terminating,
  Terminated,
  Other(String),
}

impl From<&EdcContractNegotiationState> for ContractNegotiationState {
  fn from(state: &EdcContractNegotiationState) -> Self {
    match state {
      EdcContractNegotiationState::Initial => ContractNegotiationState::Initial,
      EdcContractNegotiationState::Requesting => ContractNegotiationState::Requesting,
      EdcContractNegotiationState::Requested => ContractNegotiationState::Requested,
      EdcContractNegotiationState::Offering => ContractNegotiationState::Offering,
      EdcContractNegotiationState::Offered => ContractNegotiationState::Offered,
      EdcContractNegotiationState::Accepting => ContractNegotiationState::Accepting,
      EdcContractNegotiationState::Accepted => ContractNegotiationState::Accepted,
      EdcContractNegotiationState::Agreeing => ContractNegotiationState::Agreeing,
      EdcContractNegotiationState::Agreed => ContractNegotiationState::Agreed,
      EdcContractNegotiationState::Verifying => ContractNegotiationState::Verifying,
      EdcContractNegotiationState::Verified => ContractNegotiationState::Verified,
      EdcContractNegotiationState::Finalizing => ContractNegotiationState::Finalizing,
      EdcContractNegotiationState::Finalized => ContractNegotiationState::Finalized,
      EdcContractNegotiationState::Terminating => ContractNegotiationState::Terminating,
      EdcContractNegotiationState::Terminated => ContractNegotiationState::Terminated,
      EdcContractNegotiationState::Other(other) => {
        ContractNegotiationState::Other(other.to_string())
      }
    }
  }
}
