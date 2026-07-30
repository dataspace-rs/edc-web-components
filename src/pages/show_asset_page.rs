use crate::contexts::use_edc_connector_context;
use crate::models::AssetItem;
use edc_connector_client::EdcConnectorApiVersion;
use patternfly_yew::prelude::*;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ShowAssetPageProps {
  pub asset_id: String,
  pub on_deleted: Callback<()>,
}

#[component]
pub fn ShowAssetPage(props: &ShowAssetPageProps) -> Html {
  html!(
    <>
      <Title level={Level::H2} size={Size::XXXLarge}>{ "Show Asset" }</Title>
      <Suspense fallback="Loading ...">
        <ShowAssetPageInner
          asset_id={props.asset_id.clone()}
          on_deleted={props.on_deleted.clone()}
        />
      </Suspense>
    </>
  )
}

#[component]
pub fn ShowAssetPageInner(props: &ShowAssetPageProps) -> HtmlResult {
  let edc_connector_client = use_edc_connector_context();

  let asset = use_future_with(
    (props.asset_id.clone(), edc_connector_client.clone()),
    |properties| async move {
      let (asset_id, edc_connector_client) = (*properties).clone();

      if let Some(client) = edc_connector_client.get_client() {
        client
          .assets(EdcConnectorApiVersion::V4)
          .get(&asset_id)
          .await
          .ok()
          .map(AssetItem::from)
      } else {
        None
      }
    },
  )?;

  let asset = (*asset).clone();

  let edc_connector_context = use_edc_connector_context();

  let ondelete = use_callback(
    (
      props.asset_id.clone(),
      edc_connector_context,
      props.on_deleted.clone(),
    ),
    |_, (asset_id, edc_connector_context, on_deleted)| {
      let edc_connector_context = edc_connector_context.clone();
      let asset_id = asset_id.clone();
      let on_deleted = on_deleted.clone();

      spawn_local(async move {
        if let Some(client) = edc_connector_context.get_client()
          && client
            .assets(EdcConnectorApiVersion::V4)
            .delete(&asset_id)
            .await
            .is_ok()
        {
          on_deleted.emit(());
        }
      });
    },
  );

  if let Some(asset_item) = asset {
    let keywords = asset_item
      .keywords
      .iter()
      .map(|keyword| html! { <Label label={keyword.to_string()} color={Color::Blue} /> });

    Ok(html!(
      <>
        <DescriptionList>
          <DescriptionGroup term="ID">{ asset_item.id }</DescriptionGroup>
          <DescriptionGroup term="Title">{ asset_item.name }</DescriptionGroup>
          <DescriptionGroup term="Version">
            { asset_item.version.map(|version| version.to_string()).unwrap_or_default() }
          </DescriptionGroup>
          <DescriptionGroup term="Description">
            { asset_item.description.unwrap_or_default() }
          </DescriptionGroup>
          <DescriptionGroup term="Creator">
            { asset_item.creator.and_then(|creator| creator.name).unwrap_or_default() }
          </DescriptionGroup>
          <DescriptionGroup term="Keywords">{ for keywords }</DescriptionGroup>
          <DescriptionGroup term="Base URL">{ asset_item.base_url }</DescriptionGroup>
          <DescriptionGroup term="Proxy Path">
            <Switch disabled=true checked={asset_item.proxy_path} />
          </DescriptionGroup>
          <DescriptionGroup term="Proxy Query Parameters">
            <Switch disabled=true checked={asset_item.proxy_query_params} />
          </DescriptionGroup>
          <DescriptionGroup term="Proxy Method">
            <Switch disabled=true checked={asset_item.proxy_method} />
          </DescriptionGroup>
          <DescriptionGroup term="Proxy Body">
            <Switch disabled=true checked={asset_item.proxy_body} />
          </DescriptionGroup>
        </DescriptionList>
        <Flex>
          <FlexItem modifiers={[FlexModifier::Grow.all()]} />
          <FlexItem>
            <Button variant={ButtonVariant::Danger} onclick={ondelete.clone()}>
              { "Delete Asset" }
            </Button>
          </FlexItem>
        </Flex>
      </>
    ))
  } else {
    Ok(html!(
      format!(
      "Asset with id {} not found.",
      props.asset_id
    )
    ))
  }
}
