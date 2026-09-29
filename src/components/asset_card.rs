use crate::components::DatasetCard;
use crate::contexts::use_edc_connector_context;
use crate::models::{AssetItem, DataspaceDataset};
use edc_connector_client::EdcConnectorApiVersion;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, PartialEq, Properties)]
pub struct AssetCardProps {
  pub asset_id: String,
}

#[component]
pub fn AssetCard(props: &AssetCardProps) -> Html {
  html!(
    <Suspense fallback={html!(<Bullseye><Spinner /></Bullseye>)}>
      <AssetCardInner asset_id={props.asset_id.clone()} />
    </Suspense>
  )
}

#[component]
pub fn AssetCardInner(props: &AssetCardProps) -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();

  let results = use_future_with(
    (edc_connector_context, props.asset_id.clone()),
    |parameters| async move {
      let (edc_connector_context, asset_id) = (*parameters).clone();

      if let Some(client) = edc_connector_context.get_client() {
        client
          .assets(EdcConnectorApiVersion::V4)
          .get(&asset_id)
          .await
          .ok()
          .map(AssetItem::from)
      } else {
        None
      }
    },
  )?;

  if let Some(asset) = (*results).clone() {
    let dataset = DataspaceDataset::from(asset);

    Ok(html!(<DatasetCard {dataset} />))
  } else {
    Ok(html!(<Alert title="Asset not found" r#type={AlertType::Danger} />))
  }
}
