use crate::components::{ListAssetsGallery, SelectedFederatedCatalogOffer};
use crate::models::AssetItem;
use edc_federated_catalog_client::{FederatedCatalogClient, FederatedCatalogClientVersion};
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew::suspense::use_future_with;
use yew_oauth2::hook::use_latest_access_token;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CatalogPageProps {
  pub onselectedoffer: Callback<SelectedFederatedCatalogOffer>,
  pub on_manage_catalog: Callback<()>,
}

#[component]
pub fn CatalogPage(props: &CatalogPageProps) -> Html {
  let refresh = use_state(|| 0usize);
  let fallback = html! {
    <Bullseye>
      <Spinner size={SpinnerSize::Lg} />
    </Bullseye>
  };

  html!(
    <Stack gutter=true>
      <StackItem>
        <Split gutter=true>
          <SplitItem fill=true>
            <Title level={Level::H3} size={Size::XXLarge}>{ "Catalog" }</Title>
          </SplitItem>
        </Split>
      </StackItem>
      <StackItem>
        <Suspense {fallback}>
          <CatalogPageInner
            force_refresh={*refresh}
            onselectedoffer={props.onselectedoffer.clone()}
            on_manage_catalog={props.on_manage_catalog.clone()}
          />
        </Suspense>
      </StackItem>
    </Stack>
  )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CatalogPageInnerProps {
  pub force_refresh: usize,
  pub onselectedoffer: Callback<SelectedFederatedCatalogOffer>,
  pub on_manage_catalog: Callback<()>,
}

#[component]
pub fn CatalogPageInner(props: &CatalogPageInnerProps) -> HtmlResult {
  let latest_access_token_context = use_latest_access_token().unwrap();

  let asset_items = use_future_with(
    (latest_access_token_context.clone(), props.force_refresh),
    |parameters| async move {
      let (latest_access_token_context, _) = (*parameters).clone();

      let server_url = web_sys::window().unwrap().location().origin().unwrap();
      let federated_catalog_client = FederatedCatalogClient::new(
        reqwest::Client::new(),
        format!("{server_url}/federated-catalog"),
        latest_access_token_context.access_token(),
        FederatedCatalogClientVersion::V4,
      );

      federated_catalog_client
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
            .collect::<Vec<_>>()
        })
        .unzip()
    },
  )?;

  let (asset_items, selected_offers): (Vec<AssetItem>, Vec<SelectedFederatedCatalogOffer>) =
    (*asset_items).clone();

  let onshow = use_callback(
    (props.onselectedoffer.clone(), selected_offers),
    |dataset_id, (onselectedoffer, selected_offers)| {
      if let Some(selected_offer) = selected_offers
        .iter()
        .find(|selected_offer| selected_offer.dataset_id == dataset_id)
      {
        onselectedoffer.emit(selected_offer.clone());
      }
    },
  );

  if asset_items.is_empty() {
    Ok(html! {
      <EmptyState
        title="Empty state"
        icon={Icon::Cubes}
        primary={Action::new("Manage my catalog subscriptions", props.on_manage_catalog.reform(|_| ()))}
      >
        <div>
          <p>{ "You do not have any registered catalogs yet." }</p>
        </div>
      </EmptyState>
    })
  } else {
    Ok(html!(<ListAssetsGallery {asset_items} {onshow} />))
  }
}
