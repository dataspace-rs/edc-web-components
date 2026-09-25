use crate::components::{AssetReference, DatasetCard, DidLabel, ShowPolicy};
use crate::contexts::{use_edc_connector_context, use_edc_federated_catalog_assets_context};
use crate::models::{AssetItem, ContractAgreementItem, DataspaceDataset};
use edc_connector_client::types::contract_agreement::ContractAgreement;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ShowContractAgreementPageProps {
  #[prop_or("Contract Agreement".to_string())]
  pub title: String,
  #[prop_or(None)]
  pub tag_line: Option<String>,
  pub contract_agreement_id: String,
  #[prop_or_default]
  pub on_initiate_transfer_process: Option<Callback<ContractAgreement>>,
}

#[component]
pub fn ShowContractAgreementPage(props: &ShowContractAgreementPageProps) -> Html {
  let tag_line = props
    .tag_line
    .as_ref()
    .map(|tag_line| html!(<p>{ tag_line }</p>))
    .unwrap_or_default();

  html!(
    <>
      <Title level={Level::H2} size={Size::XXXLarge}>{ &props.title }</Title>
      { tag_line }
      <Suspense fallback={html! {<Bullseye><Spinner /></Bullseye>}}>
        <ShowContractAgreementPageInner
          contract_agreement_id={props.contract_agreement_id.clone()}
          on_initiate_transfer_process={props.on_initiate_transfer_process.clone()}
        />
      </Suspense>
    </>
  )
}

#[component]
pub fn ShowContractAgreementPageInner(props: &ShowContractAgreementPageProps) -> HtmlResult {
  let edc_connector_client = use_edc_connector_context();
  let edc_federated_catalog_assets_context = use_edc_federated_catalog_assets_context();

  let contract_agreement_and_asset = use_future_with(
    (
      props.contract_agreement_id.clone(),
      edc_connector_client.clone(),
      edc_federated_catalog_assets_context.clone(),
    ),
    |properties| async move {
      let (contract_agreement_id, edc_connector_client, edc_federated_catalog_assets_context) =
        (*properties).clone();

      if let Some(client) = edc_connector_client.get_client() {
        let contract_agreement = client
          .contract_agreements(edc_connector_client::EdcConnectorApiVersion::V4)
          .get(&contract_agreement_id)
          .await
          .ok();

        if let Some(contract_agreement) = contract_agreement {
          let asset = if let Some(edc_federated_catalog_assets_context) =
            edc_federated_catalog_assets_context
            && let Some(asset_item) =
              edc_federated_catalog_assets_context.asset(contract_agreement.asset_id())
          {
            Some(asset_item.clone())
          } else {
            client
              .assets(edc_connector_client::EdcConnectorApiVersion::V4)
              .get(contract_agreement.asset_id())
              .await
              .ok()
              .map(AssetItem::from)
          };

          (Some(contract_agreement), asset)
        } else {
          (None, None)
        }
      } else {
        (None, None)
      }
    },
  )?;

  let (contract_agreement, asset_item) = (*contract_agreement_and_asset).clone();

  if let Some(contract_agreement) = contract_agreement {
    let contract_agreement_item = ContractAgreementItem::from(contract_agreement.clone());

    let initiate_transfer_process =
      if let Some(on_initiate_transfer_process) = props.on_initiate_transfer_process.clone() {
        html!(
          <Button
            variant={ButtonVariant::Primary}
            onclick={on_initiate_transfer_process.reform(move |_| contract_agreement.clone())}
          >
            { "Initiate Transfer Process" }
          </Button>
        )
      } else {
        html!()
      };

    let asset = if let Some(asset_item) = asset_item {
      let dataset = DataspaceDataset::from(asset_item.clone());

      html!(<DatasetCard {dataset} />)
    } else {
      html!(<AssetReference asset_id={contract_agreement_item.asset_id} />)
    };

    Ok(html!(
      <Stack gutter=true>
        <StackItem>
          <Flex modifiers={[FlexModifier::Justify(Justify::Start)]}>
            <FlexItem modifiers={[FlexModifier::Flex1, FlexModifier::Align(Alignment::Start)]}>
              <Title level={Level::H4} size={Size::XLarge}>{ "Contract Properties" }</Title>
              <Card>
                <CardBody>
                  <DescriptionList mode={[DescriptionListMode::Horizontal]}>
                    <DescriptionGroup term="Id">{ contract_agreement_item.id }</DescriptionGroup>
                    <DescriptionGroup term="Contract Signing Date">
                      { contract_agreement_item.signing_date }
                    </DescriptionGroup>
                    <DescriptionGroup term="Consumer">
                      <DidLabel did={contract_agreement_item.consumer_id} />
                    </DescriptionGroup>
                    <DescriptionGroup term="Provider">
                      <DidLabel did={contract_agreement_item.provider_id} />
                    </DescriptionGroup>
                  </DescriptionList>
                </CardBody>
              </Card>
            </FlexItem>
            <FlexItem modifiers={[FlexModifier::Flex1, FlexModifier::Align(Alignment::Start)]}>
              <Title level={Level::H4} size={Size::XLarge}>{ "Policy" }</Title>
              <Card>
                <CardBody>
                  <ShowPolicy policy={contract_agreement_item.policy} />
                </CardBody>
              </Card>
            </FlexItem>
            <FlexItem modifiers={[FlexModifier::Flex1, FlexModifier::Align(Alignment::Start)]}>
              <Title level={Level::H4} size={Size::XLarge}>{ "Asset" }</Title>
              { asset }
            </FlexItem>
          </Flex>
        </StackItem>
        <StackItem>{ initiate_transfer_process }</StackItem>
      </Stack>
    ))
  } else {
    let message = format!(
      "Contract Agreement with id {} not found.",
      props.contract_agreement_id
    );

    Ok(html! { message })
  }
}
