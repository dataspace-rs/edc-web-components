use edc_identity_hub_client::models::DidWeb;
use edc_identity_hub_client::models::Identity;
use crate::models::ControlPlaneDspMetadata;

pub struct DidResolver {
    client: reqwest::Client
}

impl DidResolver {
    pub fn new(
        client: reqwest::Client
    ) -> Self {
        Self {
            client
        }
    }

    pub async fn resolve(
        &self, did: DidWeb) -> edc_identity_hub_client::Result<Identity> {
        let request_builder = self.client.get(did.url());
        let response = request_builder.send().await?;

        Ok(response.json().await?)
    }
}

pub struct ControlPlaneDspService {
    service_endpoint: String,
}

impl ControlPlaneDspService {
    pub fn new(service_endpoint: String) -> Self {
        Self { service_endpoint }
    }

    pub async fn get_metadata(&self) -> reqwest::Result<ControlPlaneDspMetadata> {
        reqwest::get(self.service_endpoint.clone())
            .await?
            .json()
            .await
    }

    pub async fn get_dsp_endpoint(&self, path: String) -> String {
        self.service_endpoint.replace("/.well-known/dspace-version", &path)
    }
}
