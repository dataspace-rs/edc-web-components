use crate::contexts::use_edc_connector_context;
use crate::models::ContractAgreementItem;
use edc_connector_client::types::contract_agreement::ContractAgreement;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ShowContractAgreementPageProps {
  pub contract_agreement_id: String,
  #[prop_or_default]
  pub on_initiate_transfer_process: Option<Callback<ContractAgreement>>,
}

#[component]
pub fn ShowContractAgreementPage(props: &ShowContractAgreementPageProps) -> Html {
  html!(
    <>
      <Title level={Level::H2} size={Size::XXXLarge}>{ "Contract Agreement" }</Title>
      <Suspense fallback="Loading ...">
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

  let contract_agreement = use_future_with(
    (
      props.contract_agreement_id.clone(),
      edc_connector_client.clone(),
    ),
    |properties| async move {
      let (contract_agreement_id, edc_connector_client) = (*properties).clone();

      if let Some(client) = edc_connector_client.get_client() {
        client
          .contract_agreements(edc_connector_client::EdcConnectorApiVersion::V4)
          .get(&contract_agreement_id)
          .await
          .ok()
      } else {
        None
      }
    },
  )?;

  let contract_agreement = (*contract_agreement).clone();

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

    Ok(html!(
      <Stack gutter=true>
        <StackItem>
          <DescriptionList mode={[DescriptionListMode::Horizontal]}>
            <DescriptionGroup term="Id">{ contract_agreement_item.id }</DescriptionGroup>
            <DescriptionGroup term="Contract Signing Date">
              { contract_agreement_item.signing_date }
            </DescriptionGroup>
            <DescriptionGroup term="Consumer Id">
              { contract_agreement_item.consumer_id }
            </DescriptionGroup>
            <DescriptionGroup term="Provider Id">
              { contract_agreement_item.provider_id }
            </DescriptionGroup>
            <DescriptionGroup term="Asset ID">
              { contract_agreement_item.asset_id }
            </DescriptionGroup>
            <DescriptionGroup term="Policy ID">
              { contract_agreement_item.policy_id }
            </DescriptionGroup>
          </DescriptionList>
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
