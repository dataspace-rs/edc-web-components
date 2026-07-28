use edc_connector_client::types::asset::Asset;

#[derive(Clone, Debug, PartialEq)]
pub struct AssetItem {
  pub id: String,
  pub name: String,
  pub base_url: String,
  pub proxy_path: bool,
  pub proxy_query_params: bool,
  pub proxy_method: bool,
  pub proxy_body: bool,
}

impl From<Asset> for AssetItem {
  fn from(asset: Asset) -> Self {
    let id = asset.id().to_string();
    let name = get_property(&asset, "name");
    let base_url = get_property(&asset, "baseUrl");
    let proxy_path = get_boolean_property(&asset, "proxyPath");
    let proxy_query_params = get_boolean_property(&asset, "proxyQueryParams");
    let proxy_method = get_boolean_property(&asset, "proxyMethod");
    let proxy_body = get_boolean_property(&asset, "proxyBody");

    AssetItem {
      id,
      name,
      base_url,
      proxy_path,
      proxy_query_params,
      proxy_method,
      proxy_body,
    }
  }
}

fn get_property(asset: &Asset, name: &str) -> String {
  asset
    .properties()
    .get::<String>(name)
    .unwrap_or_default()
    .unwrap_or_default()
}

fn get_boolean_property(asset: &Asset, name: &str) -> bool {
  asset
    .properties()
    .get::<String>(name)
    .unwrap_or_default()
    .unwrap_or_default()
    == "true"
}
