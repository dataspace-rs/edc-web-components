use crate::components::{AssetCard, DatasetCard, ShowPolicy};
use crate::contexts::use_edc_connector_context;
use crate::models::{AssetItem, ContractDefinitionItem, DataspaceDataset};
use edc_connector_client::EdcConnectorApiVersion;
use edc_connector_client::types::query::Query;
use patternfly_yew::prelude::*;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct MyPublishedOfferPageProps {
  #[prop_or("My Published Offer".to_string())]
  pub title: String,
  #[prop_or(None)]
  pub tag_line: Option<String>,
  pub asset_id: String,
}

#[component]
pub fn MyPublishedOfferPage(props: &MyPublishedOfferPageProps) -> Html {
  let tag_line = props
    .tag_line
    .as_ref()
    .map(|tag_line| html!(<p>{ tag_line }</p>))
    .unwrap_or_default();

  html!(
    <>
      <Title level={Level::H2} size={Size::XXXLarge}>{ &props.title }</Title>
      { tag_line }
      <Suspense fallback={html!{<Bullseye><Spinner/></Bullseye>}}>
        <MyPublishedOfferPageInner asset_id={props.asset_id.clone()} />
      </Suspense>
    </>
  )
}

#[component]
pub fn MyPublishedOfferPageInner(props: &MyPublishedOfferPageProps) -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();

  let refresh = use_state(|| 0usize);

  let results = use_future_with(
    (
      edc_connector_context.clone(),
      props.asset_id.clone(),
      refresh.clone(),
    ),
    |parameters| async move {
      let (edc_connector_context, asset_id, _) = (*parameters).clone();

      let query = Query::builder().limit(1000).build();

      if let Some(client) = edc_connector_context.get_client() {
        let asset = client
          .assets(EdcConnectorApiVersion::V4)
          .get(&asset_id)
          .await
          .ok()
          .map(AssetItem::from);

        let contract_definitions = client
          .contract_definitions(EdcConnectorApiVersion::V4)
          .query(query)
          .await
          .unwrap_or_default()
          .into_iter()
          .map(ContractDefinitionItem::from)
          .filter(|contract_definition_item| contract_definition_item.asset_ids.contains(&asset_id))
          .collect::<Vec<_>>();

        let mut contract_definitions_and_policies = Vec::with_capacity(contract_definitions.len());

        for contract_definition in contract_definitions.into_iter() {
          let access_policy = client
            .policies(EdcConnectorApiVersion::V4)
            .get(&contract_definition.access_policy_id)
            .await
            .ok();

          let contract_policy = client
            .policies(EdcConnectorApiVersion::V4)
            .get(&contract_definition.contract_policy_id)
            .await
            .ok();

          contract_definitions_and_policies.push((
            contract_definition,
            access_policy,
            contract_policy,
          ));
        }

        (contract_definitions_and_policies, asset)
      } else {
        (vec![], None)
      }
    },
  )?;

  let (contract_definitions_and_policies, asset_item) = (*results).clone();

  log::trace!("asset_item: {:?}", asset_item);
  log::trace!(
    "contract_definitions_and_policies: {:?}",
    contract_definitions_and_policies
  );

  let backdropper = use_backdrop();

  let unpublish_offer = use_callback(
    (
      edc_connector_context.clone(),
      refresh.clone(),
      backdropper.clone(),
    ),
    |contract_definition_id: String, (edc_connector_context, refresh, backdropper)| {
      let edc_connector_context = edc_connector_context.clone();
      let contract_definition_id = contract_definition_id.clone();
      let refresh = refresh.clone();
      let backdropper = backdropper.clone();

      spawn_local(async move {
        if let Some(client) = edc_connector_context.get_client() {
          if let Err(error) = client
            .contract_definitions(EdcConnectorApiVersion::V4)
            .delete(&contract_definition_id)
            .await
          {
            log::error!("Unable to unpublish offer: {}", error);
          } else {
            refresh.set(*refresh + 1);
            if let Some(backdropper) = backdropper {
              backdropper.close();
            }
          }
        }
      });
    },
  );

  let unpublish = use_callback(
    (backdropper.clone(), unpublish_offer.clone()),
    |contract_definition_item: ContractDefinitionItem, (backdropper, unpublish_offer)| {
      let contract_definition_id = contract_definition_item.id.clone();

      let assets = contract_definition_item
        .asset_ids
        .iter()
        .map(|asset_id| html!(<AssetCard asset_id={asset_id.clone()} />));

      if let Some(backdropper) = backdropper {
        backdropper.open(Backdrop::new(
        html!(
          <Bullseye>
            <Modal
              title={format!("Review Unpublish Offer: {}", contract_definition_item.name)}
              variant={ModalVariant::Medium}
            >
              <Stack gutter=true>
                <StackItem>
                  { "By unpublishing the offer, it will remove publication of these related asset(s):" }
                </StackItem>
                <StackItem>
                  <Gallery gutter=true>{ for assets }</Gallery>
                </StackItem>
                <StackItem>
                  <Button
                    variant={ButtonVariant::Danger}
                    onclick={unpublish_offer.reform(move |_| contract_definition_id.clone())}
                  >
                    { "Yes, unpublish this offer" }
                  </Button>
                </StackItem>
              </Stack>
            </Modal>
          </Bullseye>
        )
      ))
      }
    },
  );

  if let Some(asset_item) = asset_item {
    let dataset = DataspaceDataset::from(asset_item);

    let contract_definitions = contract_definitions_and_policies.iter().map(
      |(contract_definition_item, access_policy, contract_policy)| {
        let access_policy = access_policy.as_ref().map(|access_policy| {
          html!(<ShowPolicy policy={access_policy.policy().clone()} hide_id=true hide_kind=true />)
        });

        let contract_policy = contract_policy.as_ref().map(|contract_policy| {
          html!(<ShowPolicy policy={contract_policy.policy().clone()} hide_id=true hide_kind=true />)
        });

        let name = contract_definition_item.name.clone();
        let contract_definition_item = contract_definition_item.clone();

        html!(
          <Card>
            <CardHeader>
              <Split>
                <SplitItem fill=true>{ name }</SplitItem>
                <SplitItem>
                  <Button
                    variant={ButtonVariant::DangerSecondary}
                    onclick={unpublish.reform(move |_| contract_definition_item.clone())}
                  >
                    { "Unpublish" }
                  </Button>
                </SplitItem>
              </Split>
            </CardHeader>
            <Divider />
            <CardBody>
              <Title level={Level::H6} size={Size::Large}>{ "Access Policy" }</Title>
              { access_policy }
            </CardBody>
            <Divider />
            <CardBody>
              <Title level={Level::H6} size={Size::Large}>{ "Contract Policy" }</Title>
              { contract_policy }
            </CardBody>
          </Card>
        )
      },
    );

    Ok(html!(
      <Stack gutter=true>
        <StackItem>
          <DatasetCard {dataset} />
        </StackItem>
        <StackItem>
          <Title level={Level::H4} size={Size::XLarge}>
            { "Available Terms & Conditions for this Offer" }
          </Title>
          <Gallery gutter=true min_widths={AttrValue::from("400px").all()}>
            { for contract_definitions }
          </Gallery>
        </StackItem>
      </Stack>
    ))
  } else {
    Ok(html!(<Bullseye>{ "Asset not found" }</Bullseye>))
  }
}
