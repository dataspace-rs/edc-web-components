use edc_connector_client::types::transfer_process::TransferProcessState as EdcTransferProcessState;

#[derive(Debug, Clone, strum::Display)]
pub enum TransferProcessState {
  Initial,
  Provisioning,
  ProvisioningRequested,
  Provisioned,
  Requesting,
  Requested,
  Starting,
  Started,
  Suspending,
  Suspended,
  Resuming,
  Resumed,
  Completing,
  Completed,
  Terminating,
  Terminated,
  Deprovisioning,
  DeprovisioningRequested,
  Deprovisioned,
  Other(String),
}

impl From<&EdcTransferProcessState> for TransferProcessState {
  fn from(state: &EdcTransferProcessState) -> Self {
    match state {
      EdcTransferProcessState::Initial => TransferProcessState::Initial,
      EdcTransferProcessState::Provisioning => TransferProcessState::Provisioning,
      EdcTransferProcessState::ProvisioningRequested => TransferProcessState::ProvisioningRequested,
      EdcTransferProcessState::Provisioned => TransferProcessState::Provisioned,
      EdcTransferProcessState::Requesting => TransferProcessState::Requesting,
      EdcTransferProcessState::Requested => TransferProcessState::Requested,
      EdcTransferProcessState::Starting => TransferProcessState::Starting,
      EdcTransferProcessState::Started => TransferProcessState::Started,
      EdcTransferProcessState::Suspending => TransferProcessState::Suspending,
      EdcTransferProcessState::Suspended => TransferProcessState::Suspended,
      EdcTransferProcessState::Resuming => TransferProcessState::Resuming,
      EdcTransferProcessState::Resumed => TransferProcessState::Resumed,
      EdcTransferProcessState::Completing => TransferProcessState::Completing,
      EdcTransferProcessState::Completed => TransferProcessState::Completed,
      EdcTransferProcessState::Terminating => TransferProcessState::Terminating,
      EdcTransferProcessState::Terminated => TransferProcessState::Terminated,
      EdcTransferProcessState::Deprovisioning => TransferProcessState::Deprovisioning,
      EdcTransferProcessState::DeprovisioningRequested => {
        TransferProcessState::DeprovisioningRequested
      }
      EdcTransferProcessState::Deprovisioned => TransferProcessState::Deprovisioned,
      EdcTransferProcessState::Other(other) => TransferProcessState::Other(other.to_string()),
    }
  }
}
