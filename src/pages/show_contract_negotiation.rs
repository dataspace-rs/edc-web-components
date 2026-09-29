use crate::components::{
  AssetReference, ContractAgreementReference, ContractNegotiationStatus, DidLabel, Identifier,
};
use crate::contexts::use_edc_connector_context;
#[cfg(feature = "contract-negotiation-review")]
use edc_connector_client::types::contract_negotiation::{
  ContractNegotiationKind, ContractNegotiationState,
};
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ShowContractNegotiationPageProps {
  #[prop_or("Contract Negotiation".to_string())]
  pub title: String,
  #[prop_or(None)]
  pub tag_line: Option<String>,
  pub contract_negotiation_id: String,
  #[prop_or_default]
  pub on_contract_agreement_click: Callback<String>,
}

#[component]
pub fn ShowContractNegotiationPage(props: &ShowContractNegotiationPageProps) -> Html {
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
        <ShowContractNegotiationPageInner
          contract_negotiation_id={props.contract_negotiation_id.clone()}
          on_contract_agreement_click={props.on_contract_agreement_click.clone()}
        />
      </Suspense>
    </>
  )
}

#[component]
pub fn ShowContractNegotiationPageInner(props: &ShowContractNegotiationPageProps) -> HtmlResult {
  let edc_connector_client = use_edc_connector_context();

  let refresh = use_state(|| 0usize);

  let contract_negotiation = use_future_with(
    (
      props.contract_negotiation_id.clone(),
      edc_connector_client.clone(),
      refresh.clone(),
    ),
    |properties| async move {
      let (contract_negotiation_id, edc_connector_client, _) = (*properties).clone();

      if let Some(client) = edc_connector_client.get_client() {
        client
          .contract_negotiations(edc_connector_client::EdcConnectorApiVersion::V4)
          .get(&contract_negotiation_id)
          .await
          .ok()
      } else {
        None
      }
    },
  )?;

  let on_finalized = use_callback(refresh.setter(), |_, refresh_setter| {
    refresh_setter.set(1);
  });

  #[cfg(feature = "contract-negotiation-review")]
  let on_reviewed = use_callback(refresh.setter(), |_, refresh_setter| {
    refresh_setter.set(1);
  });

  let contract_negotiation = (*contract_negotiation).clone();

  if let Some(contract_negotiation) = contract_negotiation {
    let created_date = chrono::DateTime::from_timestamp_millis(contract_negotiation.created_at())
      .unwrap_or_default()
      .to_string();

    let contract_agreement = contract_negotiation
      .contract_agreement_id()
      .map(|contract_agreement_id| {
        html!(
          <Card>
            <CardBody>
              <Split gutter=true>
                <SplitItem>
                  <yew_icons::Icon data={yew_icons::IconData::LUCIDE_HEART_HANDSHAKE} />
                </SplitItem>
                <SplitItem fill=true>
                  <DescriptionGroup term="Contract Agreement">
                    <div class="pf-v6-u-font-weight-bold">
                      { format!("Signed {}", &created_date) }
                    </div>
                    <Identifier id={contract_agreement_id.clone()} />
                  </DescriptionGroup>
                </SplitItem>
                <SplitItem>
                  <ContractAgreementReference
                    contract_agreement_id={contract_agreement_id.clone()}
                  />
                </SplitItem>
              </Split>
            </CardBody>
          </Card>
        )
      })
      .unwrap_or_default();

    let state =
      Some(crate::models::ContractNegotiationState::from(contract_negotiation.state()).to_string())
        .map(|value| {
          let color = match value.as_str() {
            "Finalized" => Color::Green,
            "Terminated" => Color::Red,
            _ => Color::Blue,
          };

          html!(
            <DescriptionGroup term="State">
              <Label label={value} {color} />
            </DescriptionGroup>
          )
        });

    let kind =
      crate::models::ContractNegotiationKind::from(contract_negotiation.kind()).to_string();

    #[cfg(feature = "contract-negotiation-review")]
    let review = if contract_negotiation.state() == &ContractNegotiationState::Requested
      && contract_negotiation.kind() == &ContractNegotiationKind::Provider
    {
      let contract_negotiation_id = contract_negotiation.id().to_string();

      html!(
        <Alert title="Review the Contract Negotiation">
          <Suspense fallback={html!(<Bullseye><Spinner /></Bullseye>)}>
            <crate::components::ReviewContractNegotiation {contract_negotiation_id} {on_reviewed} />
          </Suspense>
        </Alert>
      )
    } else {
      html!()
    };

    #[cfg(not(feature = "contract-negotiation-review"))]
    let review = html!();

    Ok(html!(
      <Stack gutter=true>
        <StackItem>
          <Flex modifiers={[FlexModifier::Justify(Justify::Start)]}>
            <FlexItem modifiers={[FlexModifier::Flex1, FlexModifier::Align(Alignment::Start)]}>
              <Card>
                <CardTitle>
                  <Title level={Level::H2}>{ "Negotiation Properties" }</Title>
                </CardTitle>
                <CardBody>
                  <DescriptionList mode={[DescriptionListMode::Horizontal]}>
                    <DescriptionGroup term="Id">
                      <Identifier id={contract_negotiation.id().to_string()} />
                    </DescriptionGroup>
                    { state }
                    <DescriptionGroup term="Kind">{ kind }</DescriptionGroup>
                    <DescriptionGroup term="Counter Party">
                      <DidLabel
                        did={contract_negotiation.counter_party_id().clone().unwrap_or_default()}
                      />
                    </DescriptionGroup>
                    <DescriptionGroup term="Protocol">
                      <Label label={contract_negotiation.protocol().to_string()} />
                    </DescriptionGroup>
                    <DescriptionGroup term="Created At">{ created_date }</DescriptionGroup>
                  </DescriptionList>
                </CardBody>
              </Card>
            </FlexItem>
            <FlexItem modifiers={[FlexModifier::Flex1, FlexModifier::Align(Alignment::Start)]}>
              <Card>
                <CardTitle>
                  <Title level={Level::H2}>{ "Outcome" }</Title>
                </CardTitle>
                <CardBody>
                  <Flex modifiers={[FlexModifier::Column.lg()]}>
                    <FlexItem>
                      <Card>
                        <CardBody>
                          <Split gutter=true>
                            <SplitItem>
                              <yew_icons::Icon data={yew_icons::IconData::LUCIDE_BOX} />
                            </SplitItem>
                            <SplitItem fill=true>
                              <DescriptionGroup term="Asset">
                                <AssetReference
                                  asset_id={contract_negotiation.asset_id().clone().unwrap_or_default()}
                                />
                                <Identifier
                                  id={contract_negotiation.asset_id().clone().unwrap_or_default()}
                                />
                              </DescriptionGroup>
                            </SplitItem>
                          </Split>
                        </CardBody>
                      </Card>
                    </FlexItem>
                    <FlexItem>{ contract_agreement }</FlexItem>
                  </Flex>
                </CardBody>
              </Card>
            </FlexItem>
          </Flex>
        </StackItem>
        <StackItem>
          <Flex>
            <FlexItem modifiers={[FlexModifier::Flex1, FlexModifier::Align(Alignment::Start)]}>
              <Card>
                <CardTitle>
                  <Title level={Level::H2}>{ "Negotiation Process" }</Title>
                </CardTitle>
                <CardBody>
                  <ContractNegotiationStatus
                    contract_negotiation_id={props.contract_negotiation_id.clone()}
                    {on_finalized}
                  />
                </CardBody>
              </Card>
            </FlexItem>
          </Flex>
        </StackItem>
        <StackItem>{ review }</StackItem>
      </Stack>
    ))
  } else {
    Ok(html!(
      format!(
      "Contract Negotiation with id {} not found.",
      props.contract_negotiation_id
    )
    ))
  }
}
