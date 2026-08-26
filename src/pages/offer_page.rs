use crate::components::{ListAssetsGallery, SelectedFederatedCatalogOffer};
use crate::contexts::use_edc_connector_context;
use crate::models::{AssetItem, DatasetExtraFields};
use crate::services::get_dsp_endpoint;
use edc_connector_client::EdcConnectorApiVersion;
use edc_connector_client::types::Protocol;
use edc_connector_client::types::catalog::CatalogRequest;
use edc_connector_client::types::query::Query;
use edc_identity_hub_client::models::DidWeb;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct OfferPageProps {
  pub onselectedoffer: Callback<SelectedFederatedCatalogOffer>,
  pub on_new_asset: Callback<()>,
  pub on_new_policy: Callback<()>,
  pub on_new_contract: Callback<()>,
  pub participant_did: String,
}

#[component]
pub fn OfferPage(props: &OfferPageProps) -> Html {
  let refresh = use_state(|| 0usize);
  let offset = use_state(|| 0usize);
  let limit = use_state(|| 10usize);

  let on_offset = use_callback(
    (refresh.clone(), offset.setter()),
    |offset, (refresh, offset_setter)| {
      offset_setter.set(offset);
      refresh.set(**refresh + 1);
    },
  );

  let on_limit = use_callback(
    (refresh.clone(), limit.setter()),
    |limit, (refresh, limit_setter)| {
      limit_setter.set(limit);
      refresh.set(**refresh + 1);
    },
  );

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
            <Title level={Level::H3} size={Size::XXLarge}>{ "List Offers" }</Title>
          </SplitItem>
        </Split>
      </StackItem>
      <StackItem>
        <Suspense {fallback}>
          <OfferPageInner
            offset={*offset}
            limit={*limit}
            {on_offset}
            {on_limit}
            force_refresh={*refresh}
            onselectedoffer={props.onselectedoffer.clone()}
            on_new_asset={props.on_new_asset.clone()}
            on_new_policy={props.on_new_policy.clone()}
            on_new_contract={props.on_new_contract.clone()}
            participant_did={props.participant_did.clone()}
          />
        </Suspense>
      </StackItem>
    </Stack>
  )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct OfferPageInnerProps {
  pub offset: usize,
  pub limit: usize,
  pub on_offset: Callback<usize>,
  pub on_limit: Callback<usize>,
  pub force_refresh: usize,
  pub onselectedoffer: Callback<SelectedFederatedCatalogOffer>,
  pub on_new_asset: Callback<()>,
  pub on_new_policy: Callback<()>,
  pub on_new_contract: Callback<()>,
  pub participant_did: String,
}

#[component]
pub fn OfferPageInner(props: &OfferPageInnerProps) -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();

  let asset_items = use_future_with(
    (
      props.participant_did.clone(),
      edc_connector_context,
      props.limit,
      props.offset,
      props.force_refresh,
    ),
    |parameters| async move {
      let (participant_did, edc_connector_context, limit, offset, _) = (*parameters).clone();

      if let Some(did_web) = DidWeb::new(&participant_did)
        && let Some(dsp_endpoint) = get_dsp_endpoint(&did_web).await
        && let Some(client) = edc_connector_context.get_client()
      {
        let request = CatalogRequest::builder()
          .counter_party_address(dsp_endpoint.clone())
          .counter_party_id(&participant_did)
          .protocol(Protocol::default())
          .query_spec(
            Query::builder()
              .limit(limit as u32)
              .offset(offset as u32)
              .build(),
          )
          .build();

        if let Ok(response) = client
          .catalogue(EdcConnectorApiVersion::V4)
          .request::<DatasetExtraFields>(&request)
          .await
        {
          response
            .datasets()
            .iter()
            .map(|dataset| {
              let dataset_id = dataset.id().to_string();
              let asset_item = AssetItem::from(dataset);

              let selected_offer = SelectedFederatedCatalogOffer {
                originator: dsp_endpoint.clone(),
                provider_id: participant_did.clone(),
                dataset_id,
              };

              (asset_item, selected_offer)
            })
            .collect::<Vec<_>>()
            .into_iter()
            .unzip()
        } else {
          (Vec::new(), Vec::new())
        }
      } else {
        (Vec::new(), Vec::new())
      }
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
    let primary = { Action::new("Create my first asset", props.on_new_asset.reform(|_| ())) };

    let secondaries = vec![
      Action::new("Create my first policy", props.on_new_policy.reform(|_| ())),
      Action::new(
        "Create my first contract",
        props.on_new_contract.reform(|_| ()),
      ),
    ];

    Ok(html! {
      <EmptyState title="Empty state" icon={Icon::Cubes} {primary} {secondaries}>
        <div>
          <p>{ "There is no offer yet." }</p>
          <p>
            { "The first step consists of creating an asset, then a policy, and finally a contract definition." }
          </p>
          <p>{ "This results in an offer." }</p>
        </div>
      </EmptyState>
    })
  } else {
    Ok(html!(<ListAssetsGallery {asset_items} {onshow} />))
  }
}
