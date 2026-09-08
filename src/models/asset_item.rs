use crate::models::dataset_extra_fields::DatasetExtraFields;
use crate::models::{Creator, DataspaceDataset, Thumbnail};
use edc_connector_client::types::asset::Asset;
use edc_connector_client::types::catalog::Dataset;
use edc_connector_client::types::data_address::DataAddress;
use edc_connector_client::types::properties::{Properties, PropertyValue};

#[derive(Clone, Debug, PartialEq)]
pub struct AssetItem {
  pub id: String,
  pub name: String,
  pub version: Option<semver::Version>,
  pub description: Option<String>,
  pub creator: Option<Creator>,
  pub thumbnail: Option<Thumbnail>,
  pub keywords: Vec<String>,
  pub dcterm_types: Vec<String>,
  pub base_url: String,
  pub proxy_path: bool,
  pub proxy_query_params: bool,
  pub proxy_method: bool,
  pub proxy_body: bool,
  pub oauth2_client_id: String,
  pub oauth2_client_secret_key: String,
  pub oauth2_token_url: String,
  pub oauth2_scope: String,
}

impl AssetItem {
  pub fn is_filtered(&self, search: &Option<String>, dcterm_types: &[String]) -> bool {
    let text_content = format!(
      "{} {} {} {}",
      self.name.to_lowercase(),
      self.description.clone().unwrap_or_default().to_lowercase(),
      self.keywords.join(" ").to_lowercase(),
      self
        .creator
        .clone()
        .map(|creator| creator.name.unwrap_or_default().to_lowercase())
        .unwrap_or_default()
    );

    if !dcterm_types.is_empty() {
      if self
        .dcterm_types
        .iter()
        .any(|dcterm_type| dcterm_types.contains(dcterm_type))
      {
        if let Some(search) = &search {
          text_content.contains(&search.to_lowercase())
        } else {
          true
        }
      } else {
        false
      }
    } else {
      if let Some(search) = &search {
        text_content.contains(&search.to_lowercase())
      } else {
        true
      }
    }
  }
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

    let dcterm_types = asset
      .properties()
      .get_raw("http://purl.org/dc/terms/type")
      .and_then(|property_value| {
        if let Ok(value) = PropertyValue::try_from(property_value) {
          Some(vec![value])
        } else {
          serde_json::from_value::<Vec<String>>(property_value.0.clone()).ok()
        }
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

    let oauth2_client_id = asset
      .data_address()
      .property("oauth2:clientId")
      .unwrap_or_default()
      .unwrap_or_default();

    let oauth2_client_secret_key = asset
      .data_address()
      .property("oauth2:clientSecretKey")
      .unwrap_or_default()
      .unwrap_or_default();

    let oauth2_scope = asset
      .data_address()
      .property("oauth2:scope")
      .unwrap_or_default()
      .unwrap_or_default();

    let oauth2_token_url = asset
      .data_address()
      .property("oauth2:tokenUrl")
      .unwrap_or_default()
      .unwrap_or_default();

    AssetItem {
      id,
      name,
      version,
      description,
      creator,
      thumbnail,
      keywords,
      dcterm_types,
      base_url,
      proxy_path,
      proxy_query_params,
      proxy_method,
      proxy_body,
      oauth2_client_id,
      oauth2_client_secret_key,
      oauth2_token_url,
      oauth2_scope,
    }
  }
}

impl From<edc_federated_catalog_client::models::Dataset> for AssetItem {
  fn from(dataset: edc_federated_catalog_client::models::Dataset) -> Self {
    Self {
      id: dataset.id.clone(),
      name: dataset.name,
      version: dataset
        .version
        .and_then(|version| semver::Version::parse(&version).ok()),
      description: dataset.description,
      creator: dataset.creator.map(|creator| Creator {
        name: Some(creator.name),
        thumbnail: Some(Thumbnail {
          resource: Some(creator.thumbnail.resource),
        }),
      }),
      thumbnail: dataset.thumbnail.map(|thumbnail| Thumbnail {
        resource: Some(thumbnail.resource),
      }),
      keywords: dataset.keywords,
      dcterm_types: dataset.dcterm_types,
      base_url: "".to_string(),
      proxy_path: false,
      proxy_query_params: false,
      proxy_method: false,
      proxy_body: false,
      oauth2_client_id: "".to_string(),
      oauth2_client_secret_key: "".to_string(),
      oauth2_token_url: "".to_string(),
      oauth2_scope: "".to_string(),
    }
  }
}

impl From<&Dataset<DatasetExtraFields>> for AssetItem {
  fn from(dataset: &Dataset<DatasetExtraFields>) -> Self {
    let id = dataset.id().to_string();
    let extra = &dataset.extra;

    Self {
      id,
      name: extra.name.clone(),
      version: extra
        .version
        .as_ref()
        .and_then(|version| semver::Version::parse(version).ok()),
      description: extra.description.to_owned(),
      creator: extra.creator.as_ref().map(|creator| Creator {
        name: Some(creator.name.to_owned()),
        thumbnail: Some(Thumbnail {
          resource: Some(creator.thumbnail.resource.to_owned()),
        }),
      }),
      thumbnail: extra.thumbnail.as_ref().map(|thumbnail| Thumbnail {
        resource: Some(thumbnail.resource.to_owned()),
      }),
      keywords: extra.keywords.clone(),
      dcterm_types: extra.dcterm_types.clone(),
      base_url: "".to_string(),
      proxy_path: false,
      proxy_query_params: false,
      proxy_method: false,
      proxy_body: false,
      oauth2_client_id: "".to_string(),
      oauth2_client_secret_key: "".to_string(),
      oauth2_token_url: "".to_string(),
      oauth2_scope: "".to_string(),
    }
  }
}

impl From<AssetItem> for DataspaceDataset {
  fn from(asset_item: AssetItem) -> Self {
    Self {
      id: asset_item.id,
      title: asset_item.name,
      version: asset_item.version,
      comment: asset_item.description,
      thumbnail: asset_item.thumbnail,
      creator: asset_item.creator,
      keywords: asset_item.keywords,
      dcterm_types: asset_item.dcterm_types,
      policies: vec![],
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
