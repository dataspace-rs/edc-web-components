use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ControlPlaneDspMetadata {
  #[serde(rename = "protocolVersions")]
  pub protocol_versions: Vec<ProtocolVersion>,
}

#[derive(Debug, Deserialize)]
pub struct ProtocolVersion {
  pub version: String,
  pub path: String,
  pub binding: String,
}
