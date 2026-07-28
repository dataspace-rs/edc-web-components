use super::{TransferProcessKind, TransferProcessState};
use edc_connector_client::types::transfer_process::TransferProcess;

#[derive(Clone, Debug, PartialEq)]
pub struct TransferProcessItem {
  pub id: String,
  pub state: String,
  pub asset_id: String,
  pub contract_id: String,
  pub transfer_type: String,
  pub kind: String,
}

impl From<TransferProcess> for TransferProcessItem {
  fn from(transfer_process: TransferProcess) -> Self {
    let id = transfer_process.id().to_string();
    let state = TransferProcessState::from(transfer_process.state()).to_string();
    let asset_id = transfer_process.asset_id().to_string();
    let contract_id = transfer_process.contract_id().to_string();
    let transfer_type = transfer_process.transfer_type().to_string();
    let kind = TransferProcessKind::from(transfer_process.kind()).to_string();

    Self {
      id,
      state,
      asset_id,
      contract_id,
      transfer_type,
      kind,
    }
  }
}
