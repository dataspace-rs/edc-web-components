use crate::components::{
  AssetReference, ContractAgreementReference, Identifier, TransferProcessStatus,
};
use crate::contexts::use_edc_connector_context;
use edc_connector_client::types::transfer_process::{TransferProcessKind, TransferProcessState};
use patternfly_yew::prelude::*;
use web_sys::wasm_bindgen::{JsCast, JsValue};
use web_sys::{BlobPropertyBag, HtmlAnchorElement};
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ShowTransferProcessPageProps {
  #[prop_or("Transfer".to_string())]
  pub title: String,
  #[prop_or(None)]
  pub tag_line: Option<String>,
  pub transfer_process_id: String,
}

#[component]
pub fn ShowTransferProcessPage(props: &ShowTransferProcessPageProps) -> Html {
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
        <ShowTransferProcessPageInner transfer_process_id={props.transfer_process_id.clone()} />
      </Suspense>
    </>
  )
}

#[component]
pub fn ShowTransferProcessPageInner(props: &ShowTransferProcessPageProps) -> HtmlResult {
  let edc_connector_client = use_edc_connector_context();

  let refresh = use_state(|| 0usize);

  let transfer_process = use_future_with(
    (
      props.transfer_process_id.clone(),
      edc_connector_client.clone(),
      refresh.clone(),
    ),
    |properties| async move {
      let (transfer_process_id, edc_connector_client, _) = (*properties).clone();

      if let Some(client) = edc_connector_client.get_client() {
        client
          .transfer_processes(edc_connector_client::EdcConnectorApiVersion::V4)
          .get(&transfer_process_id)
          .await
          .ok()
      } else {
        None
      }
    },
  )?;

  let on_started = use_callback(refresh.setter(), |_, refresh_setter| {
    refresh_setter.set(1);
  });

  let on_finalized = use_callback(refresh.setter(), |_, refresh_setter| {
    refresh_setter.set(1);
  });

  let transfer_process = (*transfer_process).clone();

  let do_transfer = use_callback(
    edc_connector_client.clone(),
    |transfer_process_id: String, edc_connector_client| {
      let edc_connector_client = edc_connector_client.clone();
      let transfer_process_id = transfer_process_id.clone();

      spawn_local(async move {
        if let Some(client) = edc_connector_client.get_client()
          && let Ok(data_address) = client
            .edrs(edc_connector_client::EdcConnectorApiVersion::V3)
            .get_data_address(&transfer_process_id)
            .await
          && let Ok(Some(endpoint)) = data_address.property::<String>("endpoint")
          && let Ok(Some(authorization)) = data_address.property::<String>("authorization")
        {
          let client = reqwest::Client::new();

          if let Ok(response) = client
            .get(endpoint)
            .header("Authorization", authorization)
            .send()
            .await
          {
            let content_type = response
              .headers()
              .get("Content-Type")
              .map(|header_value| {
                header_value
                  .to_str()
                  .unwrap_or("application/octet-stream")
                  .to_string()
              })
              .unwrap_or("application/octet-stream".to_string());

            let extension = mime2ext::mime2ext(&content_type).unwrap_or("bin");

            log::info!("Response: {:?}", content_type);
            log::info!("Response: {:?}", response.status());

            if content_type.starts_with("text/html")
              && let Some(location) = response
                .headers()
                .get("Location")
                .map(|value| value.to_str().unwrap_or_default())
              && let Some(window) = web_sys::window()
            {
              let _ = window.open_with_url_and_target(location, "_blank");
            } else if let Ok(data) = response.bytes().await
              && let Err(error) =
                save_byte_array(&format!("data.{extension}"), &content_type, &data)
            {
              log::error!("Error saving byte array: {:?}", error);
            } else {
              log::info!("Downloaded data");
            }
          }
        }
      });
    },
  );

  let backdropper = use_backdrop();
  let show_enpoint_information = use_callback(
    (edc_connector_client.clone(), backdropper),
    |transfer_process_id: String, (edc_connector_client, backdropper)| {
      let edc_connector_client = edc_connector_client.clone();
      let backdropper = backdropper.clone();
      let transfer_process_id = transfer_process_id.clone();

      spawn_local(async move {
        if let Some(client) = edc_connector_client.get_client()
          && let Ok(data_address) = client
            .edrs(edc_connector_client::EdcConnectorApiVersion::V3)
            .get_data_address(&transfer_process_id)
            .await
          && let Ok(Some(endpoint)) = data_address.property::<String>("endpoint")
          && let Ok(Some(authorization)) = data_address.property::<String>("authorization")
          && let Some(backdropper) = backdropper
        {
          backdropper.open(Backdrop::new(html!(
            <Bullseye>
              <Modal title="Transfer Endpoint Information" variant={ModalVariant::Medium}>
                <DescriptionList>
                  <DescriptionGroup term="Endpoint URL">
                    <Clipboard readonly=true value={endpoint} />
                  </DescriptionGroup>
                  <DescriptionGroup term="Authorization Header">
                    <Clipboard readonly=true value={authorization} />
                  </DescriptionGroup>
                </DescriptionList>
              </Modal>
            </Bullseye>
          )));
        }
      });
    },
  );

  let suspend_transfer = use_callback(
    edc_connector_client.clone(),
    |transfer_process_id: String, edc_connector_client| {
      let edc_connector_client = edc_connector_client.clone();
      let transfer_process_id = transfer_process_id.clone();

      spawn_local(async move {
        if let Some(client) = edc_connector_client.get_client()
          && let Err(error) = client
            .transfer_processes(edc_connector_client::EdcConnectorApiVersion::V3)
            .suspend(&transfer_process_id, "completed")
            .await
        {
          log::error!("Error getting data address {error}");
        }
      });
    },
  );

  if let Some(transfer_process) = transfer_process {
    let transfer_process_id = transfer_process.id().to_string();
    let endpoint_transfer_process_id = transfer_process.id().to_string();
    let complete_transfer_process_id = transfer_process.id().to_string();

    let data_access = if transfer_process.state() == &TransferProcessState::Started
      && transfer_process.kind() == &TransferProcessKind::Consumer
    {
      html!(
        <Card style="border: 1px solid var(--pf-global--active-color--100, #06c);">
          <CardTitle>
            <Title level={Level::H2}>{ "Data Access" }</Title>
          </CardTitle>
          <CardBody>
            <Stack gutter=true>
              <StackItem>
                <Split gutter=true>
                  <SplitItem>
                    <yew_icons::Icon data={yew_icons::IconData::LUCIDE_CHECK} />
                  </SplitItem>
                  <SplitItem fill=true>
                    { "Download it now, or get the endpoint and token to call it from your own system." }
                  </SplitItem>
                </Split>
              </StackItem>
              <StackItem>
                <Flex>
                  <FlexItem>
                    <Button
                      variant={ButtonVariant::Primary}
                      onclick={do_transfer.reform(move |_| transfer_process_id.clone())}
                      icon={Icon::Download}
                    >
                      { "Retrieve Dataset" }
                    </Button>
                  </FlexItem>
                  <FlexItem>
                    <Button
                      variant={ButtonVariant::Primary}
                      onclick={show_enpoint_information.reform(move |_| endpoint_transfer_process_id.clone())}
                      icon={Icon::Code}
                    >
                      { "Endpoint Information" }
                    </Button>
                  </FlexItem>
                </Flex>
              </StackItem>
            </Stack>
          </CardBody>
        </Card>
      )
    } else {
      html!()
    };

    let suspend = if transfer_process.state() == &TransferProcessState::Started {
      html!(
        <FlexItem>
          <Button
            variant={ButtonVariant::Warning}
            onclick={suspend_transfer.reform(move |_| complete_transfer_process_id.clone())}
            icon={Icon::Pause}
          >
            { "Suspend Transfer" }
          </Button>
        </FlexItem>
      )
    } else {
      html!()
    };

    let correlation_id = transfer_process
      .correlation_id()
      .map(|correlation_id| html!(<Identifier id={correlation_id.to_string()} />))
      .unwrap_or(html!(
        <HelperText>
          <HelperTextItem variant={HelperTextItemVariant::Intermediate}>{ "None" }</HelperTextItem>
        </HelperText>
      ));

    let kind = crate::models::TransferProcessKind::from(transfer_process.kind()).to_string();

    let state_date = chrono::DateTime::from_timestamp_millis(transfer_process.state_timestamp())
      .unwrap_or_default()
      .to_string();

    let state = Some(
      crate::models::TransferProcessState::from(transfer_process.state()).to_string(),
    )
    .map(|value| {
      let color = match value.as_str() {
        "Started" => Color::Green,
        "Terminated" => Color::Red,
        _ => Color::Blue,
      };
      let text_help = match value.as_str() {
        "Terminated" => "at".to_string(),
        _ => "since".to_string(),
      };

      html!(
        <DescriptionGroup term="State">
          <Label label={value} {color} />
          <HelperText>
            <HelperTextItem variant={HelperTextItemVariant::Intermediate}>
              { format!("{} {}", text_help, state_date) }
            </HelperTextItem>
          </HelperText>
        </DescriptionGroup>
      )
    });

    Ok(html!(
      <Stack gutter=true>
        <StackItem>
          <Flex modifiers={[FlexModifier::Justify(Justify::Start)]}>
            <FlexItem modifiers={[FlexModifier::Flex1, FlexModifier::Align(Alignment::Stretch)]}>
              <Card full_height=true>
                <CardTitle>
                  <Title level={Level::H2}>{ "Transfer Properties" }</Title>
                </CardTitle>
                <CardBody>
                  <DescriptionList mode={[DescriptionListMode::Horizontal]}>
                    <DescriptionGroup term="Id">
                      <Identifier id={transfer_process.id().to_string()} />
                    </DescriptionGroup>
                    { state }
                    <DescriptionGroup term="Kind">{ kind }</DescriptionGroup>
                    <DescriptionGroup term="Transfer Type">
                      { transfer_process.transfer_type() }
                    </DescriptionGroup>
                    <DescriptionGroup term="Correlation Transfer ID">
                      { correlation_id }
                    </DescriptionGroup>
                  </DescriptionList>
                </CardBody>
                <CardFooter>{ suspend }</CardFooter>
              </Card>
            </FlexItem>
            <FlexItem modifiers={[FlexModifier::Flex1, FlexModifier::Align(Alignment::Stretch)]}>
              <Stack gutter=true>
                <StackItem>
                  <Card>
                    <CardTitle>
                      <Title level={Level::H2}>{ "Linked to" }</Title>
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
                                      asset_id={transfer_process.asset_id().to_string()}
                                    />
                                    <Identifier id={transfer_process.asset_id().to_string()} />
                                  </DescriptionGroup>
                                </SplitItem>
                              </Split>
                            </CardBody>
                          </Card>
                        </FlexItem>
                        <FlexItem>
                          <Card>
                            <CardBody>
                              <Split gutter=true>
                                <SplitItem>
                                  <yew_icons::Icon
                                    data={yew_icons::IconData::LUCIDE_HEART_HANDSHAKE}
                                  />
                                </SplitItem>
                                <SplitItem fill=true>
                                  <DescriptionGroup term="Contract Agreement">
                                    <div class="pf-v6-u-font-weight-bold">{ "Agreement" }</div>
                                    <Identifier id={transfer_process.contract_id().to_string()} />
                                  </DescriptionGroup>
                                </SplitItem>
                                <SplitItem>
                                  <ContractAgreementReference
                                    contract_agreement_id={transfer_process.contract_id().to_string()}
                                  />
                                </SplitItem>
                              </Split>
                            </CardBody>
                          </Card>
                        </FlexItem>
                      </Flex>
                    </CardBody>
                  </Card>
                </StackItem>
                <StackItem>{ data_access }</StackItem>
              </Stack>
            </FlexItem>
          </Flex>
        </StackItem>
        <StackItem>
          <Flex>
            <FlexItem modifiers={[FlexModifier::Flex1, FlexModifier::Align(Alignment::Start)]}>
              <Card>
                <CardTitle>
                  <Title level={Level::H2}>{ "Transfer Process" }</Title>
                </CardTitle>
                <CardBody>
                  <TransferProcessStatus
                    transfer_process_id={props.transfer_process_id.clone()}
                    {on_started}
                    {on_finalized}
                  />
                </CardBody>
              </Card>
            </FlexItem>
          </Flex>
        </StackItem>
      </Stack>
    ))
  } else {
    Ok(html!(
      format!(
      "Transfer Process with id {} not found.",
      props.transfer_process_id
    )
    ))
  }
}

fn save_byte_array(name: &str, mime_type: &str, data: &[u8]) -> Result<(), JsValue> {
  use web_sys::{Blob, Url, js_sys::Uint8Array};

  // Build file data & metadata
  let props = BlobPropertyBag::new();
  props.set_type(mime_type);

  let blob = Blob::new_with_u8_array_sequence_and_options(
    &JsValue::from(vec![Uint8Array::new_from_slice(data)]),
    &props,
  )?;

  // Add the link element
  let document = web_sys::window()
    .and_then(|window| window.document())
    .ok_or(JsValue::null())?;

  let link = document.create_element("a")?;

  // Set link attributes
  let url = Url::create_object_url_with_blob(&blob)?;
  link.set_attribute("href", &url)?;
  link.set_attribute("download", name)?;

  link.dyn_into::<HtmlAnchorElement>()?.click();
  Url::revoke_object_url(&url)?;

  Ok(())
}
