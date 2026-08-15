use chrono::{DateTime, Utc};
use edc_identity_hub_client::models::Credential;

#[derive(Clone, Debug, PartialEq)]
pub struct VerifiableCredential {
  pub id: String,
  pub created_at: u64,
  pub timestamp: u64,
  pub issuer_id: String,
  pub holder_id: String,
  pub issuance_date: DateTime<Utc>,
  pub expiration_date: DateTime<Utc>,
}

impl From<Credential> for VerifiableCredential {
  fn from(credential: Credential) -> Self {
    VerifiableCredential {
      id: credential.id,
      created_at: credential.created_at.timestamp() as u64,
      timestamp: credential.timestamp.timestamp() as u64,
      issuer_id: credential.issuer_id,
      holder_id: credential.holder_id,
      issuance_date: credential.verifiable_credential.credential.issuance_date,
      expiration_date: credential.verifiable_credential.credential.expiration_date,
    }
  }
}
