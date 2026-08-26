use edc_identity_hub_client::models::{DidWeb, Identity};

pub struct DidResolver {
  client: reqwest::Client,
}

impl DidResolver {
  pub fn new(client: reqwest::Client) -> Self {
    Self { client }
  }

  pub async fn resolve(&self, did: &DidWeb) -> edc_identity_hub_client::Result<Identity> {
    let request_builder = self.client.get(did.url());
    let response = request_builder.send().await?;

    Ok(response.json().await?)
  }
}
