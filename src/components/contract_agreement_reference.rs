use crate::contexts::{RedirectionAction, use_edc_connector_context, use_redirection_context};
use crate::models::ContractAgreementItem;
use edc_connector_client::EdcConnectorApiVersion;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ContractAgreementReferenceProps {
  pub contract_agreement_id: String,
}

#[component]
pub fn ContractAgreementReference(props: &ContractAgreementReferenceProps) -> Html {
  let redirection_context = use_redirection_context();

  let onclick = redirection_context
    .map(|redirection_context| {
      let contract_agreement_id = props.contract_agreement_id.clone();

      redirection_context
        .redirect_to()
        .reform(move |_| RedirectionAction::ContractAgreement(contract_agreement_id.clone()))
    })
    .unwrap_or_default();

  html! {
    html!(<Button variant={ButtonVariant::InlineLink} {onclick}>
      <Flex>
      <FlexItem modifiers={[FlexModifier::Align(Alignment::Center)]}>
        <yew_icons::Icon data={yew_icons::IconData::LUCIDE_HEART_HANDSHAKE} />

      </FlexItem>
      <FlexItem modifiers={[FlexModifier::Align(Alignment::Center)]}>
        { "Agreement" }
      </FlexItem>
      </Flex>
      </Button>)
  }
}

#[component]
pub fn ContractAgreementReferenceInner(props: &ContractAgreementReferenceProps) -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();

  let contract_agreement = use_future_with(
    (
      props.contract_agreement_id.clone(),
      edc_connector_context.clone(),
    ),
    async move |parameters| {
      let (contract_agreement_id, edc_connector_context) = (*parameters).clone();

      if let Some(edc_connector_client) = edc_connector_context.get_client() {
        edc_connector_client
          .contract_agreements(EdcConnectorApiVersion::V4)
          .get(&contract_agreement_id)
          .await
          .ok()
          .map(ContractAgreementItem::from)
      } else {
        None
      }
    },
  )?;

  let _contract_agreement = (*contract_agreement).clone();

  Ok(html!())
}
