use crate::components::{ListAssetsGallery, SelectedFederatedCatalogOffer};
use crate::models::AssetItem;
use edc_federated_catalog_client::{FederatedCatalogClient, FederatedCatalogClientVersion};
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew::suspense::use_future_with;
use yew_oauth2::hook::use_latest_access_token;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct FederatedCatalogPageProps {
  pub on_selected_offer: Callback<SelectedFederatedCatalogOffer>,
  pub on_manage_catalog: Callback<()>,
  #[prop_or("/federated-catalog".to_string())]
  pub federated_catalog_endpoint: String,
  #[prop_or_default]
  pub display_search: bool,
  #[prop_or_default]
  pub search: Option<String>,
  #[prop_or_default]
  pub dcterm_types: Vec<String>,
}

#[component]
pub fn FederatedCatalogPage(props: &FederatedCatalogPageProps) -> Html {
  let refresh = use_state(|| 0usize);
  let fallback = html! {
    <Bullseye>
      <Spinner size={SpinnerSize::Lg} />
    </Bullseye>
  };

  let search = use_state(String::new);

  let onchange = use_callback(search.setter(), move |value, search_setter| {
    search_setter.set(value);
  });

  let onclear = use_callback(search.setter(), move |_, search_setter| {
    search_setter.set(String::new());
  });

  let (search_component, search) = if props.display_search {
    let inner = html!(
      <StackItem>
        <SearchInput placeholder="Search" value={(*search).clone()} {onchange} {onclear} />
      </StackItem>
    );

    let search = if (*search).is_empty() {
      None
    } else {
      Some((*search).clone())
    };

    (inner, search)
  } else {
    (html!(), props.search.clone())
  };

  html!(
    <Stack gutter=true>
      <StackItem>
        <Split gutter=true>
          <SplitItem fill=true>
            <Title level={Level::H3} size={Size::XXLarge}>{ "Catalog" }</Title>
            <p>
              { "Explore and access available data offers from your list of trusted and followed participants." }
            </p>
          </SplitItem>
        </Split>
      </StackItem>
      { search_component }
      <StackItem>
        <Suspense {fallback}>
          <FederatedCatalogPageInner
            force_refresh={*refresh}
            on_selected_offer={props.on_selected_offer.clone()}
            on_manage_catalog={props.on_manage_catalog.clone()}
            federated_catalog_endpoint={props.federated_catalog_endpoint.clone()}
            {search}
            dcterm_types={props.dcterm_types.clone()}
          />
        </Suspense>
      </StackItem>
    </Stack>
  )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct FederatedCatalogPageInnerProps {
  pub force_refresh: usize,
  pub on_selected_offer: Callback<SelectedFederatedCatalogOffer>,
  pub on_manage_catalog: Callback<()>,
  #[prop_or("/federated-catalog".to_string())]
  pub federated_catalog_endpoint: String,
  #[prop_or_default]
  pub search: Option<String>,
  #[prop_or_default]
  pub dcterm_types: Vec<String>,
}

#[component]
pub fn FederatedCatalogPageInner(props: &FederatedCatalogPageInnerProps) -> HtmlResult {
  let latest_access_token_context = use_latest_access_token().unwrap();

  let asset_items = use_future_with(
    (
      props.federated_catalog_endpoint.clone(),
      props.search.clone(),
      props.dcterm_types.clone(),
      latest_access_token_context.clone(),
      props.force_refresh,
    ),
    |parameters| async move {
      let (federated_catalog_endpoint, search, dcterm_types, latest_access_token_context, _) =
        (*parameters).clone();

      let server_url = web_sys::window().unwrap().location().origin().unwrap();
      let federated_catalog_client = FederatedCatalogClient::new(
        reqwest::Client::new(),
        format!("{server_url}{}", federated_catalog_endpoint),
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
            .filter(|(asset_item, _)| asset_item.is_filtered(&search, &dcterm_types))
            .collect::<Vec<_>>()
        })
        .unzip()
    },
  )?;

  let (asset_items, selected_offers): (Vec<AssetItem>, Vec<SelectedFederatedCatalogOffer>) =
    (*asset_items).clone();

  let onshow = use_callback(
    (props.on_selected_offer.clone(), selected_offers),
    |dataset_id, (on_selected_offer, selected_offers)| {
      if let Some(selected_offer) = selected_offers
        .iter()
        .find(|selected_offer| selected_offer.dataset_id == dataset_id)
      {
        on_selected_offer.emit(selected_offer.clone());
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
