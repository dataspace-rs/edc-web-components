use crate::models::{Creator, Thumbnail};
use edc_connector_client::types::asset::Asset;
use edc_connector_client::types::data_address::DataAddress;
use edc_connector_client::types::properties::Properties;

#[derive(Clone, Debug, PartialEq)]
pub struct AssetItem {
  pub id: String,
  pub name: String,
  pub version: Option<semver::Version>,
  pub description: Option<String>,
  pub creator: Option<Creator>,
  pub thumbnail: Option<Thumbnail>,
  pub keywords: Vec<String>,
  pub base_url: String,
  pub proxy_path: bool,
  pub proxy_query_params: bool,
  pub proxy_method: bool,
  pub proxy_body: bool,
}

impl From<Asset> for AssetItem {
  fn from(asset: Asset) -> Self {
    let id = asset.id().to_string();
    let name = get_property(asset.properties(), "name");
    let description = asset
      .properties()
      .get::<String>("http://www.w3.org/2000/01/rdf-schema#comment")
      .unwrap_or_default();

    let version = asset
      .properties()
      .get::<String>("http://www.w3.org/ns/dcat#version")
      .unwrap_or_default()
      .and_then(|version| semver::Version::parse(version.as_str()).ok());

    let creator = asset
      .properties()
      .get_raw("http://purl.org/dc/terms/creator")
      .and_then(|property_value| serde_json::from_value::<Creator>(property_value.0.clone()).ok());

    let thumbnail = asset
      .properties()
      .get_raw("http://xmlns.com/foaf/0.1/thumbnail")
      .and_then(|property_value| {
        serde_json::from_value::<Thumbnail>(property_value.0.clone()).ok()
      });

    let keywords = asset
      .properties()
      .get_raw("http://www.w3.org/ns/dcat#keyword")
      .and_then(|property_value| {
        serde_json::from_value::<Vec<String>>(property_value.0.clone()).ok()
      })
      .unwrap_or_default();

    let base_url = asset
      .data_address()
      .property("baseUrl")
      .unwrap_or_default()
      .unwrap_or_default();

    let proxy_path = get_boolean_property(asset.data_address(), "proxyPath");
    let proxy_query_params = get_boolean_property(asset.data_address(), "proxyQueryParams");
    let proxy_method = get_boolean_property(asset.data_address(), "proxyMethod");
    let proxy_body = get_boolean_property(asset.data_address(), "proxyBody");

    AssetItem {
      id,
      name,
      version,
      description,
      creator,
      thumbnail,
      keywords,
      base_url,
      proxy_path,
      proxy_query_params,
      proxy_method,
      proxy_body,
    }
  }
}

fn get_property(properties: &Properties, name: &str) -> String {
  properties
    .get::<String>(name)
    .unwrap_or_default()
    .unwrap_or_default()
}

fn get_boolean_property(data_address: &DataAddress, name: &str) -> bool {
  data_address
    .property::<String>(name)
    .unwrap_or_default()
    .unwrap_or_default()
    == "true"
}
