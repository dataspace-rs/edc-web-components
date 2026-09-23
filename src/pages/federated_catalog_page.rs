use crate::components::{ListAssetsGallery, SelectedFederatedCatalogOffer};
use crate::contexts::{
  EdcFederatedCatalogAssetsContextProvider, EdcFederatedCatalogAssetsFetcher,
  use_edc_federated_catalog_assets_context,
};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct FederatedCatalogPageProps {
  #[prop_or("Catalog".to_string())]
  pub title: String,
  #[prop_or("Explore and access available data offers from your list of trusted and followed participants.".to_string())]
  pub description: String,
  pub on_selected_offer: Callback<SelectedFederatedCatalogOffer>,
  #[prop_or_default]
  pub on_manage_catalog: Option<Callback<()>>,
  #[prop_or("/federated-catalog".to_string())]
  pub federated_catalog_endpoint: String,
  #[prop_or_default]
  pub display_search: bool,
  #[prop_or_default]
  pub search: Option<String>,
  #[prop_or_default]
  pub dcterm_types: Vec<String>,
  #[prop_or("No such offer".to_string())]
  pub empty_title: String,
  #[prop_or("You may not have registered participants.".to_string())]
  pub empty_description: String,
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
            <Title level={Level::H3} size={Size::XXLarge}>{ &props.title }</Title>
            <p>{ &props.description }</p>
          </SplitItem>
        </Split>
      </StackItem>
      { search_component }
      <StackItem>
        <EdcFederatedCatalogAssetsContextProvider>
          <Suspense {fallback}>
            <EdcFederatedCatalogAssetsFetcher
              federated_catalog_endpoint={props.federated_catalog_endpoint.clone()}
              force_refresh={*refresh}
              {search}
              dcterm_types={props.dcterm_types.clone()}
            />
            <FederatedCatalogPageInner
              on_selected_offer={props.on_selected_offer.clone()}
              on_manage_catalog={props.on_manage_catalog.clone()}
              empty_title={props.empty_title.clone()}
              empty_description={props.empty_description.clone()}
            />
          </Suspense>
        </EdcFederatedCatalogAssetsContextProvider>
      </StackItem>
    </Stack>
  )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct FederatedCatalogPageInnerProps {
  pub on_selected_offer: Callback<SelectedFederatedCatalogOffer>,
  #[prop_or_default]
  pub on_manage_catalog: Option<Callback<()>>,
  #[prop_or("No such offer".to_string())]
  pub empty_title: String,
  #[prop_or("You may not have registered participants.".to_string())]
  pub empty_description: String,
}

#[component]
pub fn FederatedCatalogPageInner(props: &FederatedCatalogPageInnerProps) -> Html {
  let edc_federated_catalog_assets_context = use_edc_federated_catalog_assets_context()
    .expect("EdcFederatedCatalogAssetsContextProvider missing");

  let asset_items = edc_federated_catalog_assets_context.asset_items().to_vec();
  let selected_offers = edc_federated_catalog_assets_context
    .selected_offers()
    .to_vec();

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
    let primary = props.on_manage_catalog.as_ref().map(|on_manage_catalog| {
      Action::new(
        "Manage my catalog subscriptions",
        on_manage_catalog.reform(|_| ()),
      )
    });

    html! {
      <EmptyState title={props.empty_title.to_string()} {primary}>
        <div>
          <p>{ &props.empty_description }</p>
        </div>
      </EmptyState>
    }
  } else {
    html!(<ListAssetsGallery {asset_items} {onshow} />)
  }
}
