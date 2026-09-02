use edc_connector_client::types::ExtraTokenFields;
use edc_federated_catalog_client::models::{Creator, Thumbnail};
use serde::Deserialize;
use serde_with::serde_as;

#[serde_as]
#[derive(Deserialize, Debug, Clone)]
pub struct DatasetExtraFields {
  #[serde(rename = "name", alias = "edc:name")]
  pub name: String,
  #[serde(rename = "contenttype", alias = "edc:contenttype")]
  pub content_type: String,
  #[serde(alias = "dct:title", default)]
  pub title: Option<String>,
  #[serde(alias = "http://www.w3.org/2000/01/rdf-schema#comment", default)]
  pub description: Option<String>,
  #[serde(alias = "dcat:version", default)]
  pub version: Option<String>,
  #[serde(alias = "dct:creator", default)]
  pub creator: Option<Creator>,
  #[serde(alias = "http://xmlns.com/foaf/0.1/thumbnail", default)]
  pub thumbnail: Option<Thumbnail>,
  #[serde(alias = "dcat:keyword", default)]
  pub keywords: Vec<String>,
  #[serde(alias = "dct:type", alias = "http://purl.org/dc/terms/type", default)]
  pub dcterm_types: Vec<String>,
}

impl ExtraTokenFields for DatasetExtraFields {}
