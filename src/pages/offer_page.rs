use crate::components::{ListAssetsGallery, SelectedFederatedCatalogOffer};
use crate::contexts::{use_edc_connector_context, use_edc_identity_hub_context};
use crate::models::AssetItem;
use crate::services::{ControlPlaneDspService, DidResolver};
use edc_connector_client::EdcConnectorApiVersion;
use edc_connector_client::types::catalog::CatalogRequest;
use edc_connector_client::types::query::Query;
use edc_connector_client::types::{ExtraTokenFields, Protocol};
use edc_federated_catalog_client::models::{Creator, Thumbnail};
use edc_identity_hub_client::models::DidWeb;
use patternfly_yew::prelude::*;
use serde::Deserialize;
use serde_with::serde_as;
use yew::prelude::*;
use yew::suspense::use_future_with;
use yew_oauth2::hook::use_latest_access_token;

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

impl ExtraTokenFields for CatalogExtraFields {
}


#[derive(Clone, Debug, PartialEq, Properties)]
pub struct OfferPageProps {
  pub onselectedoffer: Callback<SelectedFederatedCatalogOffer>,
  pub on_new_asset: Callback<()>,
  pub on_new_policy: Callback<()>,
  pub on_new_contract: Callback<()>
}

#[component]
pub fn OfferPage(props: &OfferPageProps) -> Html {
  let refresh = use_state(|| 0usize);
  let offset = use_state(|| 0usize);
  let limit = use_state(|| 10usize);

  let onoffset = use_callback(
    (refresh.clone(), offset.setter()),
    |offset, (refresh, offset_setter)| {
      offset_setter.set(offset);
      refresh.set(**refresh + 1);
    },
  );

  let onlimit = use_callback(
    (refresh.clone(), limit.setter()),
    |limit, (refresh, limit_setter)| {
      limit_setter.set(limit);
      refresh.set(**refresh + 1);
    },
  );

  let on_new_asset = use_callback(props.on_new_asset.clone(), |_, new_asset| {
    new_asset.emit(());
  });
  let on_new_policy = use_callback(props.on_new_policy.clone(), |_, new_policy| {
    new_policy.emit(());
  });
  let on_new_contract = use_callback(props.on_new_contract.clone(), |_, new_contract| {
    new_contract.emit(());
  });

  let fallback = html! {<Bullseye><Spinner size={SpinnerSize::Lg} /></Bullseye>};

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
            {onoffset}
            {onlimit}
            force_refresh={*refresh}
            onselectedoffer={props.onselectedoffer.clone()}
            on_new_asset={on_new_asset}
            on_new_policy={on_new_policy}
            on_new_contract={on_new_contract}
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
  pub onoffset: Callback<usize>,
  pub onlimit: Callback<usize>,
  pub force_refresh: usize,
  pub onselectedoffer: Callback<SelectedFederatedCatalogOffer>,
  pub on_new_asset: Callback<()>,
  pub on_new_policy: Callback<()>,
  pub on_new_contract: Callback<()>
}

#[component]
pub fn OfferPageInner(props: &OfferPageInnerProps) -> HtmlResult {
  let latest_access_token_context = use_latest_access_token().unwrap();
  let edc_connector_context = use_edc_connector_context();
  let edc_identity_hub_context = use_edc_identity_hub_context();

  let onclick_asset = use_callback(props.on_new_asset.clone(), |_, on_new_asset| {
    on_new_asset.emit(());
  });
  let onclick_policy = use_callback(props.on_new_policy.clone(), |_, other_new_policy| {
    other_new_policy.emit(());
  });
  let onclick_contract = use_callback(props.on_new_contract.clone(), |_, other_new_contract| {
    other_new_contract.emit(());
  });

  let asset_items = use_future_with(
    (
      edc_identity_hub_context,
      edc_connector_context,
      props.limit,
      props.offset,
      props.force_refresh
    ),
    |parameters| async move {
      let (edc_identity_hub_context, edc_connector_context, limit, offset, _) = (*parameters).clone();

      let dsp: Option<String> = match DidWeb::new(&edc_identity_hub_context.participant_did())
      {
        Some(did_web) => match DidResolver::new(reqwest::Client::new()).resolve(did_web).await
        {
          Ok(did_data) => {
            match did_data.get_identity_services("DataService").first()
                .and_then(|url| Some(ControlPlaneDspService::new(url.service_endpoint.clone())))
            {
              Some(service) => {
                match service.get_metadata().await
                {
                  Ok(response) => {
                    Option::from(service.get_dsp_endpoint(response.protocol_versions.first().unwrap().path.clone()).await)
                  }
                  _ => None
                }
              }
              _ => None
            }
          }
          _ => None
        }
        _ => None
      };

      let request = CatalogRequest::builder()
          .counter_party_address(dsp.clone().unwrap_or_default())
          .counter_party_id(edc_identity_hub_context.participant_did())
          .protocol(Protocol::new("dataspace-protocol-http:2025-1"))
          .query_spec(
            Query::builder()
                .limit(limit as u32)
                .offset(offset as u32)
                .build(),
          )
          .build();

      if let Some(client) = edc_connector_context.get_client() {
        match client
            .catalogue(EdcConnectorApiVersion::V4)
            .request::<CatalogExtraFields>(&request)
            .await
        {
          Ok(response) => {
            response.datasets()
                .into_iter()
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
                    originator: String::from(dsp.clone().unwrap_or_default()),
                    provider_id: String::from(edc_identity_hub_context.participant_did()),
                    dataset_id: String::from(dataset.id()),
                  };

                  (asset_item, selected_offer)
                })
                .collect::<Vec<_>>()
                .into_iter()
                .unzip()
          }
          _ => (Vec::new(), Vec::new())
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
    Ok(html!{
      <EmptyState
            title="Empty state"
            icon={Icon::Cubes}
            primary={Action::new("Create my first asset", onclick_asset)}
            secondaries={vec![
                Action::new("Create my first policy", onclick_policy),
                Action::new("Create my first contract", onclick_contract)
            ]}
            >
            <div>
              <p>{"There is no offer yet."}</p>
              <p>{"The first step consists of creating an asset, then a policy, and finally a contract definition."}</p>
              <p>{"This results in an offer."}</p>
            </div>
      </EmptyState>
    })
  } else {
    Ok(html!(
      <ListAssetsGallery {asset_items} {onshow} />
    ))
  }
}
