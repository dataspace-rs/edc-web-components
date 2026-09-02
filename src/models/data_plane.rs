use edc_connector_client::types::dataplane::{DataPlaneInstance, DataPlaneInstanceState};

#[derive(Debug, Clone, PartialEq)]
pub struct DataPlane {
  pub id: String,
  pub url: String,
  pub allowed_source_types: Vec<String>,
  pub allowed_dest_types: Vec<String>,
  pub allowed_transfer_types: Vec<String>,
  pub state: DataPlaneInstanceState,
}

impl From<DataPlaneInstance> for DataPlane {
  fn from(data_plane: DataPlaneInstance) -> Self {
    Self {
      id: data_plane.id().to_string(),
      url: data_plane.url().to_string(),
      allowed_source_types: data_plane.allowed_source_types().to_vec(),
      allowed_dest_types: data_plane.allowed_dest_types().to_vec(),
      allowed_transfer_types: data_plane.allowed_transfer_types().to_vec(),
      state: data_plane.state().clone(),
    }
  }
}
