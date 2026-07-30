use edc_connector_client::types::policy::Policy;
use semver::Version;
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq)]
pub struct DataspaceDataset {
  pub id: String,
  pub title: String,
  pub version: Option<Version>,
  pub comment: Option<String>,
  pub thumbnail: Option<Thumbnail>,
  pub creator: Option<Creator>,
  pub keywords: Vec<String>,
  pub policies: Vec<Policy>,
  pub dcterm_types: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct Creator {
  #[serde(alias = "http://xmlns.com/foaf/0.1/name")]
  pub name: Option<String>,
  #[serde(alias = "http://xmlns.com/foaf/0.1/thumbnail")]
  pub thumbnail: Option<Thumbnail>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct Thumbnail {
  #[serde(alias = "rdf:resource")]
  pub resource: Option<String>,
}
