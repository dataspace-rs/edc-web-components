use strum::{EnumProperty, IntoEnumIterator};

#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  strum::EnumProperty,
  strum::EnumIter,
  strum::Display,
  strum::EnumString,
)]
pub enum DataSourceTypes {
  #[strum(props(Label = "HttpData"))]
  HttpData,
}

pub fn datasource_default() -> Vec<(String, String)> {
  DataSourceTypes::iter()
    .map(|data_source_types: DataSourceTypes| {
      (
        data_source_types.to_string(),
        data_source_types
          .get_str("Label")
          .unwrap_or_default()
          .to_string(),
      )
    })
    .collect()
}
