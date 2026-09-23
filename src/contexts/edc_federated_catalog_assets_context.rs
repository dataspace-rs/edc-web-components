use crate::components::SelectedFederatedCatalogOffer;
use crate::models::AssetItem;
use edc_federated_catalog_client::{FederatedCatalogClient, FederatedCatalogClientVersion};
use std::{ops::Deref, rc::Rc};
use yew::prelude::*;
use yew::suspense::use_future_with;
use yew_oauth2::hook::use_latest_access_token;

#[derive(Debug, PartialEq, Clone)]
pub enum EdcFederatedCatalogAssetsAction {
  SetAssetsAndSelectedOffers(Vec<(AssetItem, SelectedFederatedCatalogOffer)>),
}

#[derive(Clone, PartialEq)]
pub struct EdcFederatedCatalogAssetsContextState {
  asset_items: Vec<AssetItem>,
  selected_offers: Vec<SelectedFederatedCatalogOffer>,
}

impl EdcFederatedCatalogAssetsContextState {
  pub fn asset(&self, asset_id: &str) -> Option<&AssetItem> {
    self
      .asset_items
      .iter()
      .find(|asset_item| asset_item.id == asset_id)
  }

  pub fn asset_items(&self) -> &[AssetItem] {
    &self.asset_items
  }

  pub fn selected_offers(&self) -> &[SelectedFederatedCatalogOffer] {
    &self.selected_offers
  }
}

impl Reducible for EdcFederatedCatalogAssetsContextState {
  type Action = EdcFederatedCatalogAssetsAction;

  fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
    let mut new_self = self.deref().clone();

    let EdcFederatedCatalogAssetsAction::SetAssetsAndSelectedOffers(assets_and_selected_offers) =
      action;

    let (assets, selected_offers) = assets_and_selected_offers.into_iter().unzip();

    new_self.asset_items = assets;
    new_self.selected_offers = selected_offers;

    new_self.into()
  }
}

#[derive(Properties, PartialEq)]
pub struct EdcFederatedCatalogAssetsContextProviderProps {
  #[prop_or_default]
  pub children: Html,
}

#[component]
pub fn EdcFederatedCatalogAssetsContextProvider(
  props: &EdcFederatedCatalogAssetsContextProviderProps,
) -> Html {
  let did_resolver_context = use_reducer(move || EdcFederatedCatalogAssetsContextState {
    asset_items: vec![],
    selected_offers: vec![],
  });

  html! {
    <ContextProvider<EdcFederatedCatalogAssetsContext> context={did_resolver_context}>
      { props.children.clone() }
    </ContextProvider<EdcFederatedCatalogAssetsContext >>
  }
}

pub type EdcFederatedCatalogAssetsContext = UseReducerHandle<EdcFederatedCatalogAssetsContextState>;

#[hook]
pub fn use_edc_federated_catalog_assets_context() -> Option<EdcFederatedCatalogAssetsContext> {
  use_context::<EdcFederatedCatalogAssetsContext>()
}

#[derive(Properties, PartialEq)]
pub struct EdcFederatedCatalogAssetsFetcherProps {
  #[prop_or("/federated-catalog".to_string())]
  pub federated_catalog_endpoint: String,
  #[prop_or_default]
  pub search: Option<String>,
  #[prop_or_default]
  pub dcterm_types: Vec<String>,
  #[prop_or_default]
  pub force_refresh: usize,
}

#[component]
pub fn EdcFederatedCatalogAssetsFetcher(
  props: &EdcFederatedCatalogAssetsFetcherProps,
) -> HtmlResult {
  let latest_access_token_context = use_latest_access_token().unwrap();
  let edc_federated_catalog_assets_context = use_edc_federated_catalog_assets_context();

  let _ = use_future_with(
    (
      props.force_refresh,
      props.federated_catalog_endpoint.clone(),
      props.search.clone(),
      props.dcterm_types.clone(),
      latest_access_token_context.clone(),
      edc_federated_catalog_assets_context.clone(),
    ),
    |parameters| async move {
      let (
        _,
        federated_catalog_endpoint,
        search,
        dcterm_types,
        latest_access_token_context,
        edc_federated_catalog_assets_context,
      ) = (*parameters).clone();

      if let Some(edc_federated_catalog_assets_context) = edc_federated_catalog_assets_context {
        let server_url = web_sys::window().unwrap().location().origin().unwrap();
        let federated_catalog_client = FederatedCatalogClient::new(
          reqwest::Client::new(),
          format!("{server_url}{}", federated_catalog_endpoint),
          latest_access_token_context.access_token(),
          FederatedCatalogClientVersion::V4,
        );

        let assets_and_selected_offers = federated_catalog_client
          .list_offers()
          .await
          .unwrap_or_default()
          .iter()
          .flat_map(|federated_catalog_offer| {
            federated_catalog_offer
              .dataset
              .clone()
              .into_iter()
              .map(|dataset| {
                let dataset_id = dataset.id.clone();
                let asset_item = AssetItem::from(dataset);

                let selected_offer = SelectedFederatedCatalogOffer {
                  originator: federated_catalog_offer.originator.clone(),
                  provider_id: federated_catalog_offer.participant_id.id.clone(),
                  dataset_id,
                };

                (asset_item, selected_offer)
              })
              .filter(|(asset_item, _)| asset_item.is_filtered(&search, &dcterm_types))
              .collect::<Vec<_>>()
          })
          .collect::<Vec<_>>();

        edc_federated_catalog_assets_context.dispatch(
          EdcFederatedCatalogAssetsAction::SetAssetsAndSelectedOffers(assets_and_selected_offers),
        );
      }
    },
  )?;

  Ok(html!())
}
