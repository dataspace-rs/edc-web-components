use crate::components::{ListFederatedCatalogOffers, SelectedFederatedCatalogOffer};
use edc_federated_catalog_client::{FederatedCatalogClient, FederatedCatalogClientVersion};
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew::suspense::use_future_with;
use yew_oauth2::hook::use_latest_access_token;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CatalogPageProps {
  pub onselectedoffer: Callback<SelectedFederatedCatalogOffer>,
}

#[component]
pub fn CatalogPage(props: &CatalogPageProps) -> Html {
  let refresh = use_state(|| 0usize);

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
        <Suspense>
          <CatalogPageInner
            force_refresh={*refresh}
            onselectedoffer={props.onselectedoffer.clone()}
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
}

#[component]
pub fn CatalogPageInner(props: &CatalogPageInnerProps) -> HtmlResult {
  let latest_access_token_context = use_latest_access_token().unwrap();

  let federated_catalog_offers = use_future_with(
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
    },
  )?;

  let federated_catalog_offers = (*federated_catalog_offers).clone();

  Ok(html!(
    <ListFederatedCatalogOffers
      {federated_catalog_offers}
      onselectedoffer={props.onselectedoffer.clone()}
    />
  ))
}
