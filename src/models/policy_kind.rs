use edc_connector_client::types::policy::PolicyKind as EdcPolicyKind;

#[derive(Debug, Clone, strum::Display)]
pub enum PolicyKind {
  Set,
  Offer,
  Agreement,
}

impl From<&EdcPolicyKind> for PolicyKind {
  fn from(kind: &EdcPolicyKind) -> Self {
    match kind {
      EdcPolicyKind::Set => PolicyKind::Set,
      EdcPolicyKind::Offer => PolicyKind::Offer,
      EdcPolicyKind::Agreement => PolicyKind::Agreement,
    }
  }
}
