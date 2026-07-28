use edc_connector_client::types::transfer_process::TransferProcessKind as EdcTransferProcessKind;

#[derive(Debug, Clone, strum::Display)]
pub enum TransferProcessKind {
  Consumer,
  Provider,
}
impl From<&EdcTransferProcessKind> for TransferProcessKind {
  fn from(kind: &EdcTransferProcessKind) -> Self {
    match kind {
      EdcTransferProcessKind::Consumer => TransferProcessKind::Consumer,
      EdcTransferProcessKind::Provider => TransferProcessKind::Provider,
    }
  }
}
