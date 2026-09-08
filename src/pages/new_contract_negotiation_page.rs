use crate::components::PolicyCard;
use crate::contexts::use_edc_connector_context;
use crate::models::{DatasetExtraFields, PolicyDefinitionItem};
use base64::prelude::*;
use edc_connector_client::EdcConnectorApiVersion;
use edc_connector_client::types::Protocol;
use edc_connector_client::types::catalog::{Dataset, DatasetRequest};
use edc_connector_client::types::contract_negotiation::ContractRequest;
use edc_connector_client::types::policy::{
  AtomicConstraint, Constraint, LeftOperand, Operator, Permission, Policy, PolicyKind, Target,
};
use patternfly_yew::prelude::*;
use std::fmt::Debug;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct NewContractNegotiationPageProps {
  pub originator: String,
  pub provider_id: String,
  pub dataset_id: String,
  pub on_contract_negotiation_id: Callback<String>,
}

#[component]
pub fn NewContractNegotiationPage(props: &NewContractNegotiationPageProps) -> Html {
  html!(
    <Stack gutter=true>
      <StackItem>
        <Split gutter=true>
          <SplitItem fill=true>
            <Title level={Level::H3} size={Size::XXLarge}>{ "Negotiate a Contract" }</Title>
            <p>
              { "Start a new negotiation on the selected data offer and proposing terms to establish a binding sharing contract." }
            </p>
          </SplitItem>
        </Split>
      </StackItem>
      <StackItem>
        <Suspense>
          <NewContractNegotiationPageInner
            originator={props.originator.clone()}
            provider_id={props.provider_id.clone()}
            dataset_id={props.dataset_id.clone()}
            on_contract_negotiation_id={props.on_contract_negotiation_id.clone()}
          />
        </Suspense>
      </StackItem>
    </Stack>
  )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct NewContractNegotiationPageInnerProps {
  pub originator: String,
  pub provider_id: String,
  pub dataset_id: String,
  pub on_contract_negotiation_id: Callback<String>,
}

#[component]
pub fn NewContractNegotiationPageInner(props: &NewContractNegotiationPageInnerProps) -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();

  let catalog_dataset = use_future_with(
    (
      edc_connector_context.clone(),
      props.originator.clone(),
      props.provider_id.clone(),
      props.dataset_id.clone(),
    ),
    |parameters| async move {
      let (edc_connector_context, originator, provider_id, dataset_id) = (*parameters).clone();

      if let Some(edc_connector_client) = edc_connector_context.get_client() {
        let dataset_request = DatasetRequest::builder()
          .id(dataset_id.clone())
          .counter_party_address(originator)
          .counter_party_id(provider_id)
          .protocol(Protocol::default())
          .build();

        edc_connector_client
          .catalogue(EdcConnectorApiVersion::V4)
          .dataset(&dataset_request)
          .await
          .ok()
          .map(|catalog_dataset: Dataset<DatasetExtraFields>| {
            (
              catalog_dataset.id().to_string(),
              catalog_dataset.extra.name.clone(),
              catalog_dataset.offers().to_vec(),
              catalog_dataset.extra.description.clone(),
              catalog_dataset.extra.creator.clone(),
            )
          })
      } else {
        None
      }
    },
  )?;

  let catalog_dataset = (*catalog_dataset).clone();

  let selected_offer = use_state_eq(|| None);
  let signing = use_state(|| false);

  let onchange = use_callback(selected_offer.clone(), |policy: Policy, selected_offer| {
    selected_offer.set(Some(policy))
  });

  let onclick = use_callback(
    (
      edc_connector_context.clone(),
      catalog_dataset.clone(),
      selected_offer.clone(),
      props.originator.clone(),
      props.provider_id.clone(),
      signing.setter(),
      props.on_contract_negotiation_id.clone(),
    ),
    |_,
     (
      edc_connector_context,
      catalog_dataset,
      selected_offer,
      originator,
      provider_id,
      signing_setter,
      on_contract_negotiation_id,
    )| {
      let edc_connector_context = edc_connector_context.clone();
      if let Some(policy) = (**selected_offer).clone()
        && let Some(catalog_dataset) = catalog_dataset
      {
        let originator = originator.clone();
        let provider_id = provider_id.clone();
        let asset_id = catalog_dataset.0.to_string();

        signing_setter.set(true);
        let signing_setter = signing_setter.clone();
        let on_contract_negotiation_id = on_contract_negotiation_id.clone();

        spawn_local(async move {
          if let Some(edc_client) = edc_connector_context.get_client() {
            let counter_party_address = originator.clone();
            let counter_party_id = provider_id.clone();

            log::info!("counter_party_address: {}", counter_party_address);
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

              match edc_client
                .contract_negotiations(EdcConnectorApiVersion::V4)
                .initiate(&new_contract_request)
                .await
              {
                Ok(contract_negotiation_id) => {
                  log::info!(
                    "Contract negotiation initiated: {}",
                    contract_negotiation_id.id()
                  );

                  on_contract_negotiation_id.emit(contract_negotiation_id.id().to_string());
                }
                Err(error) => {
                  log::error!("Error initiating contract negotiation: {}", error);
                }
              }
            };
          }

          signing_setter.set(false);
        })
      }
    },
  );

  if let Some(catalog_dataset) = catalog_dataset {
    let provider_id = props.provider_id.clone();
    let asset_id = catalog_dataset.0.clone();
    let asset_name = catalog_dataset.1.clone();
    let policies = catalog_dataset.2;
    let description = catalog_dataset.3;
    let creator = catalog_dataset.4;
    let disabled = *signing;

    let offers = policies
      .iter()
      .filter_map(|policy: &Policy| {
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
      .map(|(policy, _offer_id): (&Policy, String)| {
        let onchange = onchange.clone();
        let policy_definition_item = PolicyDefinitionItem::from(policy);
        let selected = (*selected_offer).as_ref() == Some(policy);

        let policy = policy.clone();

        html!(
          <PolicyCard
            {policy_definition_item}
            {selected}
            {disabled}
            on_click={onchange.reform(move |_| policy.clone())}
          />
        )
      });

    let disabled = selected_offer.is_none() || *signing;

    let details = description
      .map(|value| html!(<DescriptionGroup term="Description">{ value }</DescriptionGroup>))
      .unwrap_or_default();

    let provider = creator
      .map(|value| {
        html!(
          <>
            <DescriptionGroup term="Provider">{ value.name }</DescriptionGroup>
            <DescriptionGroup term="Provider ID">{ provider_id }</DescriptionGroup>
          </>
        )
      })
      .unwrap_or_default();

    Ok(html!(
      <>
        <DescriptionList>
          { provider }
          <DescriptionGroup term="Asset ID">{ asset_id }</DescriptionGroup>
          <DescriptionGroup term="Asset Name">{ asset_name }</DescriptionGroup>
          { details }
          <DescriptionGroup term="Offers">
            <Gallery gutter=true>{ for offers }</Gallery>
          </DescriptionGroup>
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
