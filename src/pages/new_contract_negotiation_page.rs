use crate::contexts::use_edc_connector_context;
use base64::prelude::*;
use edc_connector_client::EdcConnectorApiVersion;
use edc_connector_client::types::catalog::CatalogRequest;
use edc_connector_client::types::contract_negotiation::ContractRequest;
use edc_connector_client::types::policy::{
  AtomicConstraint, Constraint, LeftOperand, Operator, Permission, Policy, PolicyKind, Target,
};
use edc_connector_client::types::query::Query;
use edc_connector_client::types::{ExtraTokenFields, Protocol};
use edc_federated_catalog_client::{FederatedCatalogClient, FederatedCatalogClientVersion};
use patternfly_yew::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::suspense::use_future_with;
use yew_oauth2::hook::use_latest_access_token;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct NewContractNegotiationPageProps {
  pub originator: String,
  pub provider_id: String,
  pub dataset_id: String,
}

#[component]
pub fn NewContractNegotiationPage(props: &NewContractNegotiationPageProps) -> Html {
  html!(
    <Stack gutter=true>
      <StackItem>
        <Split gutter=true>
          <SplitItem fill=true>
            <Title level={Level::H3} size={Size::XXLarge}>{ "New Contract Negotiation" }</Title>
          </SplitItem>
        </Split>
      </StackItem>
      <StackItem>
        <Card>
          <CardBody>
            <Suspense>
              <NewContractNegotiationPageInner
                originator={props.originator.clone()}
                provider_id={props.provider_id.clone()}
                dataset_id={props.dataset_id.clone()}
              />
            </Suspense>
          </CardBody>
        </Card>
      </StackItem>
    </Stack>
  )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct NewContractNegotiationPageInnerProps {
  pub originator: String,
  pub provider_id: String,
  pub dataset_id: String,
}

#[component]
pub fn NewContractNegotiationPageInner(props: &NewContractNegotiationPageInnerProps) -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();
  let latest_access_token_context = use_latest_access_token().unwrap();

  let federated_catalog_dataset = use_future_with(
    (
      latest_access_token_context,
      props.originator.clone(),
      props.provider_id.clone(),
      props.dataset_id.clone(),
      edc_connector_context.clone(),
    ),
    |parameters| async move {
      let (latest_access_token_context, originator, provider_id, dataset_id, edc_connector_context) =
        (*parameters).clone();

      if let Some(edc_client) = edc_connector_context.get_client() {
        // let originator = originator.clone();
        // let provider_id = provider_id.clone();

        {
          let originator =
            "https://controlplane.participant-li1.demo.luminvent.com/dsp/2025-1".to_string();
          let provider_id =
            "did:web:participant-li1.demo.luminvent.com:participant-li1".to_string();

          let edc_client = edc_client.clone();
          spawn_local(async move {
            let query = Query::builder()
              // .limit(*limit as u32)
              // .offset(*offset as u32)
              .build();

            let catalogue_request = CatalogRequest::builder()
              .counter_party_address(originator.clone())
              .counter_party_id(provider_id.clone())
              .protocol(Protocol::new("dataspace-protocol-http:2025-1"))
              .query_spec(query)
              .build();

            #[derive(Debug, Serialize, Deserialize)]
            struct ExtraDatasetFields {}
            impl ExtraTokenFields for ExtraDatasetFields {}

            if let Ok(catalog) = edc_client
              .catalogue(EdcConnectorApiVersion::V4)
              .request::<ExtraDatasetFields>(&catalogue_request)
              .await
            {
              log::warn!("Catalog: {:?}", catalog);
            }
          });
        }

        let originator =
          "https://controlplane.participant-li2.demo.luminvent.com/dsp/2025-1".to_string();
        let provider_id = "did:web:participant-li2.demo.luminvent.com:participant-li2".to_string();

        spawn_local(async move {
          let query = Query::builder()
            // .limit(*limit as u32)
            // .offset(*offset as u32)
            .build();

          let catalogue_request = CatalogRequest::builder()
            .counter_party_address(originator.clone())
            .counter_party_id(provider_id.clone())
            .protocol(Protocol::new("dataspace-protocol-http:2025-1"))
            .query_spec(query)
            .build();

          #[derive(Debug, Serialize, Deserialize)]
          struct ExtraDatasetFields {}
          impl ExtraTokenFields for ExtraDatasetFields {}

          if let Ok(catalog) = edc_client
            .catalogue(EdcConnectorApiVersion::V4)
            .request::<ExtraDatasetFields>(&catalogue_request)
            .await
          {
            log::warn!("Catalog: {:?}", catalog);
          }
        })
      }

      let server_url = web_sys::window().unwrap().location().origin().unwrap();
      let federated_catalog_client = FederatedCatalogClient::new(
        reqwest::Client::new(),
        format!("{server_url}/federated-catalog"),
        latest_access_token_context.access_token(),
        FederatedCatalogClientVersion::V4,
      );

      let federated_catalog_offers = federated_catalog_client
        .list_offers()
        .await
        .unwrap_or_default();

      federated_catalog_offers
        .into_iter()
        .find_map(|federated_catalog_offer| {
          if federated_catalog_offer.originator != originator
            && federated_catalog_offer.participant_id.id != provider_id
          {
            None
          } else {
            federated_catalog_offer
              .dataset
              .into_iter()
              .find(|dataset| dataset.id == dataset_id)
          }
        })
    },
  )?;

  let federated_catalog_dataset = (*federated_catalog_dataset).clone();

  let selected_offer = use_state_eq(|| None);

  let onchange = use_callback(selected_offer.clone(), |policy: Policy, selected_offer| {
    selected_offer.set(Some(policy))
  });

  let onclick = use_callback(
    (
      edc_connector_context.clone(),
      federated_catalog_dataset.clone(),
      selected_offer.clone(),
      props.originator.clone(),
      props.provider_id.clone(),
    ),
    |_,
     (
      edc_connector_context,
      federated_catalog_dataset,
      selected_offer,
      originator,
      provider_id,
    )| {
      let edc_connector_context = edc_connector_context.clone();
      if let Some(policy) = (**selected_offer).clone()
        && let Some(federated_catalog_dataset) = federated_catalog_dataset
      {
        let originator = originator.clone();
        let provider_id = provider_id.clone();
        let asset_id = federated_catalog_dataset.id.clone();

        spawn_local(async move {
          if let Some(edc_client) = edc_connector_context.get_client() {
            //let counter_party_address = federated_catalog_offer.service.endpoint_url.clone();
            let counter_party_address = originator.clone();
            let counter_party_id = provider_id.clone();

            log::warn!("counter_party_address: {}", counter_party_address);
            let policy: Policy = policy.clone();

            if let Some(id) = policy.id() {
              let policy_builder = Policy::builder()
                .assigner(provider_id)
                .id(id)
                .kind(PolicyKind::Offer)
                .permissions(
                  policy
                    .permissions()
                    .iter()
                    .map(|permission| {
                      Permission::builder()
                        .action(edc_connector_client::types::policy::Action::Simple(
                          "use".to_string(),
                        ))
                        .constraints(
                          permission
                            .constraints()
                            .iter()
                            .map(|constraint| match constraint {
                              Constraint::Atomic(atomic_constraint) => {
                                let left_operand = match &atomic_constraint.left_operand {
                                  LeftOperand::Simple(simple) => {
                                    LeftOperand::Simple(simple.to_string())
                                  }
                                  LeftOperand::Id { id } => LeftOperand::Simple(id.to_string()),
                                };

                                let operator = match &atomic_constraint.operator {
                                  Operator::Simple(simple) => Operator::Simple(simple.to_string()),
                                  Operator::Id { id } => Operator::Simple(
                                    id.to_string().replace("http://www.w3.org/ns/odrl/2/", ""),
                                  ),
                                };

                                Constraint::Atomic(AtomicConstraint {
                                  left_operand,
                                  operator,
                                  right_operand: atomic_constraint.right_operand.clone(),
                                })
                              }
                              Constraint::MultiplicityConstraint(multiplicity_constraint) => {
                                Constraint::MultiplicityConstraint(multiplicity_constraint.clone())
                              }
                            })
                            .collect(),
                        )
                        .build()
                    })
                    .collect(),
                )
                .prohibitions(policy.prohibitions().to_vec())
                .obligations(policy.obligations().to_vec())
                .target(Target::Simple(asset_id));

              let policy = policy_builder.build();

              let new_contract_request = ContractRequest::builder()
                .protocol(Protocol::new("dataspace-protocol-http:2025-1"))
                .counter_party_address(&counter_party_address)
                .counter_party_id(counter_party_id)
                .policy(policy)
                .build();

              if let Ok(contract_negotiation_id) = edc_client
                .contract_negotiations(EdcConnectorApiVersion::V4)
                .initiate(&new_contract_request)
                .await
              {
                log::warn!(
                  "Contract negotiation initiated: {}",
                  contract_negotiation_id.id()
                );
              }
            };
          }
        })
      }
    },
  );

  if let Some(federated_catalog_dataset) = federated_catalog_dataset {
    let provider_id = props.provider_id.clone();
    let asset_id = federated_catalog_dataset.id.clone();
    let asset_name = federated_catalog_dataset.name.clone();
    let policies = federated_catalog_dataset.has_policy;
    let selected_offer = selected_offer.clone();

    let offers = policies
      .iter()
      .filter_map(|policy| {
        if let Some(offer_id) = policy.id().and_then(|policy| policy.split(':').next()) {
          BASE64_STANDARD
            .decode(offer_id)
            .ok()
            .and_then(|offer_id| String::from_utf8(offer_id).ok())
            .map(|offer_id| (policy, offer_id))
        } else {
          None
        }
      })
      .map(|(policy, offer_id)| {
        let onchange = onchange.clone();
        let policy = policy.clone();

        html!(
          <Radio
            name="offer-id"
            checked={(*selected_offer).as_ref() == Some(&policy)}
            onchange={onchange.reform(move |_| policy.clone())}
          >
            <span>{ offer_id.to_string() }</span>
          </Radio>
        )
      });

    let disabled = selected_offer.is_none();

    Ok(html!(
      <>
        <DescriptionList>
          <DescriptionGroup term="Provider ID">{ provider_id }</DescriptionGroup>
          <DescriptionGroup term="Asset ID">{ asset_id }</DescriptionGroup>
          <DescriptionGroup term="Asset Name">{ asset_name }</DescriptionGroup>
          <DescriptionGroup term="Offer IDs">{ for offers }</DescriptionGroup>
        </DescriptionList>
        <Split gutter=true>
          <SplitItem fill=true />
          <SplitItem>
            <Button variant={ButtonVariant::Primary} icon={Icon::Check} {disabled} {onclick}>
              { "Sign" }
            </Button>
          </SplitItem>
        </Split>
      </>
    ))
  } else {
    Ok(html!({ "The offer is not available." }))
  }
}
