use crate::components::TransferProcessStatus;
use crate::contexts::use_edc_connector_context;
use edc_connector_client::types::transfer_process::TransferProcessState;
use patternfly_yew::prelude::*;
use web_sys::{BlobPropertyBag, HtmlAnchorElement};
use web_sys::wasm_bindgen::{JsCast, JsValue};
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ShowTransferProcessPageProps {
  pub transfer_process_id: String,
}

#[component]
pub fn ShowTransferProcessPage(props: &ShowTransferProcessPageProps) -> Html {
  html!(
    <>
      <Title level={Level::H2} size={Size::XXXLarge}>{ "Transfer Process" }</Title>
      <Suspense fallback="Loading ...">
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

            if let Ok(data) = response.bytes().await {
              if let Err(error) = save_byte_array(&format!("data.{extension}"), &content_type, &data) {
                log::error!("Error saving byte array: {:?}", error);
              }
            }
          }
        }
      });
    },
  );

  if let Some(transfer_process) = transfer_process {
    let transfer_proces_id = transfer_process.id().to_string();

    let start_edrs = if transfer_process.state() == &TransferProcessState::Started {
      html!(
        <Button
          variant={ButtonVariant::Primary}
          onclick={do_transfer.reform(move |_| transfer_proces_id.clone())}
        >
          { "Retrieve Dataset" }
        </Button>
      )
    } else {
      html!()
    };

    Ok(html!(
      <Stack gutter=true>
        <StackItem>
          <DescriptionList mode={[DescriptionListMode::Horizontal]}>
            <DescriptionGroup term="Id">{ transfer_process.id() }</DescriptionGroup>
            <DescriptionGroup term="Contract Agreement Id">
              { transfer_process.contract_id() }
            </DescriptionGroup>
            <DescriptionGroup term="Correlation Contract Agreement ID">
              { transfer_process.correlation_id() }
            </DescriptionGroup>
            <DescriptionGroup term="Asset ID">{ transfer_process.asset_id() }</DescriptionGroup>
            <DescriptionGroup term="Transfer Type">
              { transfer_process.transfer_type() }
            </DescriptionGroup>
          </DescriptionList>
        </StackItem>
        <StackItem>
          <TransferProcessStatus
            transfer_process_id={props.transfer_process_id.clone()}
            {on_started}
            {on_finalized}
          />
        </StackItem>
        <StackItem>{ start_edrs }</StackItem>
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
    &JsValue::from(vec![Uint8Array::new_from_slice(&data)]),
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
