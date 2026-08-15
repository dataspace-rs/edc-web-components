use crate::components::{ListAssetsGallery, SelectedFederatedCatalogOffer};
use crate::contexts::{use_edc_connector_context, use_edc_identity_hub_context};
use crate::models::AssetItem;
use crate::services::DidResolver;
use edc_connector_client::EdcConnectorApiVersion;
use edc_connector_client::types::catalog::CatalogRequest;
use edc_connector_client::types::query::Query;
use edc_connector_client::types::{ExtraTokenFields, Protocol};
use edc_federated_catalog_client::models::{Creator, Thumbnail};
use edc_identity_hub_client::models::{DidWeb, IdentityServiceType};
use patternfly_yew::prelude::*;
use serde::Deserialize;
use serde_with::serde_as;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[serde_as]
#[derive(Deserialize, Debug, Clone)]
pub struct CatalogExtraFields {
  #[serde(rename = "name", alias = "edc:name")]
  pub name: String,
  #[serde(rename = "contenttype", alias = "edc:contenttype")]
  pub content_type: String,
  #[serde(alias = "dct:title", default)]
  pub title: Option<String>,
  #[serde(alias = "http://www.w3.org/2000/01/rdf-schema#comment", default)]
  pub description: Option<String>,
  #[serde(alias = "dcat:version", default)]
  pub version: Option<String>,
  #[serde(alias = "dct:creator", default)]
  pub creator: Option<Creator>,
  #[serde(alias = "http://xmlns.com/foaf/0.1/thumbnail", default)]
  pub thumbnail: Option<Thumbnail>,
  #[serde(alias = "dcat:keyword", default)]
  pub keywords: Vec<String>,
}

impl ExtraTokenFields for CatalogExtraFields {}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct OfferPageProps {
  pub onselectedoffer: Callback<SelectedFederatedCatalogOffer>,
  pub on_new_asset: Callback<()>,
  pub on_new_policy: Callback<()>,
  pub on_new_contract: Callback<()>,
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
}

#[component]
pub fn OfferPageInner(props: &OfferPageInnerProps) -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();
  let edc_identity_hub_context = use_edc_identity_hub_context();

  let asset_items = use_future_with(
    (
      edc_identity_hub_context,
      edc_connector_context,
      props.limit,
      props.offset,
      props.force_refresh,
    ),
    |parameters| async move {
      let (edc_identity_hub_context, edc_connector_context, limit, offset, _) =
        (*parameters).clone();

      if let Some(did_web) = DidWeb::new(edc_identity_hub_context.participant_did())
        && let Ok(did_data) = DidResolver::new(reqwest::Client::new())
          .resolve(did_web)
          .await
        && let Some(identity_service) = did_data
          .get_identity_services(IdentityServiceType::DataService)
          .first()
        && let Some(dataspace_service_client) = identity_service.get_dataspace_service_client()
        && let Some(dsp_endpoint) = dataspace_service_client.get_first_service_endpoint().await
        && let Some(client) = edc_connector_context.get_client()
      {
        let request = CatalogRequest::builder()
          .counter_party_address(dsp_endpoint.clone())
          .counter_party_id(edc_identity_hub_context.participant_did())
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
          .request::<CatalogExtraFields>(&request)
          .await
        {
          response
            .datasets()
            .iter()
            .map(|dataset| {
              let extra = dataset.extra.clone();
              let asset_item = AssetItem {
                id: dataset.id().parse().unwrap(),
                name: dataset.extra.name.clone(),
                version: extra
                  .version
                  .and_then(|version| semver::Version::parse(&version).ok()),
                description: extra.description,
                creator: extra.creator.map(|creator| crate::models::Creator {
                  name: Some(creator.name),
                  thumbnail: Some(crate::models::Thumbnail {
                    resource: Some(creator.thumbnail.resource),
                  }),
                }),
                thumbnail: extra.thumbnail.map(|thumbnail| crate::models::Thumbnail {
                  resource: Some(thumbnail.resource),
                }),
                keywords: extra.keywords,
                base_url: "".to_string(),
                proxy_path: false,
                proxy_query_params: false,
                proxy_method: false,
                proxy_body: false,
              };

              let selected_offer = SelectedFederatedCatalogOffer {
                originator: dsp_endpoint.clone(),
                provider_id: edc_identity_hub_context.participant_did().to_string(),
                dataset_id: dataset.id().to_string(),
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
