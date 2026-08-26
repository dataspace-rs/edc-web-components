use crate::services::DidResolver;
use edc_identity_hub_client::models::{DidWeb, IdentityServiceType};

pub async fn get_dsp_endpoint(did_web: &DidWeb) -> Option<String> {
  if let Ok(identity) = DidResolver::new(reqwest::Client::new())
    .resolve(did_web)
    .await
    && let Some(identity_service) = identity
      .get_identity_services(IdentityServiceType::DataService)
      .first()
    && let Some(dataspace_service_client) = identity_service.get_dataspace_service_client()
    && let Some(dsp_endpoint) = dataspace_service_client.get_first_service_endpoint().await
  {
    Some(dsp_endpoint)
  } else {
    None
  }
}
