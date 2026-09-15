use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Participant {
  pub did: String,
  pub name: String,
  #[serde(default)]
  pub logo: Option<String>,
}
