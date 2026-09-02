use crate::contexts::use_edc_connector_context;
use crate::models::AssetItem;
use edc_connector_client::EdcConnectorApiVersion;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct AssetReferenceProps {
  pub asset_id: String,
  pub on_click: Callback<()>,
}

#[component]
pub fn AssetReference(props: &AssetReferenceProps) -> Html {
  html! {
    <Suspense fallback={html! {<Bullseye><Spinner size={SpinnerSize::Sm} /></Bullseye>}}>
      <AssetReferenceInner asset_id={props.asset_id.clone()} on_click={props.on_click.clone()} />
    </Suspense>
  }
}

#[component]
pub fn AssetReferenceInner(props: &AssetReferenceProps) -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();

  let asset = use_future_with(
    (props.asset_id.clone(), edc_connector_context.clone()),
    async move |parameters| {
      let (asset_id, edc_connector_context) = (*parameters).clone();

      if let Some(edc_connector_client) = edc_connector_context.get_client() {
        edc_connector_client
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

  let asset = (*asset).clone();

  let label = if let Some(asset) = asset {
    asset.name
  } else {
    props.asset_id.clone()
  };

  Ok(html!(
    <Button variant={ButtonVariant::InlineLink} onclick={props.on_click.reform(|_| ())}>
      { label }
    </Button>
  ))
}
