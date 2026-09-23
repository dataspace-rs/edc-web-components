use crate::components::DatasetCard;
use crate::contexts::{
  RedirectionAction, use_edc_connector_context, use_edc_federated_catalog_assets_context,
  use_redirection_context,
};
use crate::models::{AssetItem, DataspaceDataset};
use edc_connector_client::EdcConnectorApiVersion;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct AssetReferenceProps {
  pub asset_id: String,
}

#[component]
pub fn AssetReference(props: &AssetReferenceProps) -> Html {
  html! {
    <Suspense fallback={html! {<Bullseye><Spinner size={SpinnerSize::Sm} /></Bullseye>}}>
      <AssetReferenceInner asset_id={props.asset_id.clone()} />
    </Suspense>
  }
}

#[component]
pub fn AssetReferenceInner(props: &AssetReferenceProps) -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();
  let backdropper = use_backdrop();
  let edc_federated_catalog_assets_context = use_edc_federated_catalog_assets_context();
  let redirection_context = use_redirection_context();

  let asset = use_future_with(
    (
      props.asset_id.clone(),
      edc_federated_catalog_assets_context.clone(),
      edc_connector_context.clone(),
    ),
    async move |parameters| {
      let (asset_id, edc_federated_catalog_assets_context, edc_connector_context) =
        (*parameters).clone();

      if let Some(edc_federated_catalog_assets_context) = edc_federated_catalog_assets_context
        && let Some(asset_item) = edc_federated_catalog_assets_context.asset(&asset_id)
      {
        Some(asset_item.clone())
      } else {
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
      }
    },
  )?;

  let asset = (*asset).clone();

  let label = if let Some(asset) = &asset {
    asset.name.clone()
  } else {
    props.asset_id.clone()
  };

  let onclick = use_callback(
    (
      backdropper.clone(),
      redirection_context.clone(),
      asset.clone(),
      props.asset_id.clone(),
    ),
    |_, (backdropper, redirection_context, asset, asset_id)| {
      let asset_id = asset_id.clone();

      if let Some(backdropper) = backdropper
        && let Some(asset) = asset
      {
        let dataspace_dataset = DataspaceDataset::from(asset.clone());

        let button = if let Some(redirection_context) = redirection_context {
          let backdropper = backdropper.clone();

          html!(
            <Button
              variant={ButtonVariant::Primary}
              icon={Icon::Eye}
              onclick={redirection_context.redirect_to().reform(move |_| {
            backdropper.close();
            RedirectionAction::Asset(asset_id.clone())
          })}
            >
              { "Show" }
            </Button>
          )
        } else {
          html!()
        };

        backdropper.open(Backdrop::new(html!(
          <Bullseye>
            <Modal title="Asset" variant={ModalVariant::Small}>
              <Bullseye>
                <Stack gutter=true>
                  <StackItem>
                    <DatasetCard dataset={dataspace_dataset} />
                  </StackItem>
                  <StackItem>{ button }</StackItem>
                </Stack>
              </Bullseye>
            </Modal>
          </Bullseye>
        )));
      } else {
        if let Some(redirection_context) = redirection_context {
          redirection_context
            .redirect_to()
            .emit(RedirectionAction::Asset(asset_id));
        }
      }
    },
  );

  let variant = if backdropper.is_some() || redirection_context.is_some() {
    ButtonVariant::InlineLink
  } else {
    ButtonVariant::Plain
  };

  Ok(html!(<Button {variant} {onclick}>{ label }</Button>))
}
