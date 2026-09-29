use crate::components::ListAssetsGallery;
use crate::contexts::use_edc_connector_context;
use crate::models::AssetItem;
use edc_connector_client::EdcConnectorApiVersion;
use edc_connector_client::types::query::Query;
use patternfly_yew::prelude::*;
use std::collections::HashSet;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct MyPublishedOffersPageProps {
  pub on_new_contract_definition: Callback<()>,
  #[prop_or("My Published Offers".to_string())]
  pub title: String,
  #[prop_or(Some("Active data offerings that can be presented to consumers".to_string()))]
  pub tag_line: Option<String>,
  #[prop_or(AttrValue::Static("Show Offers"))]
  pub button_label: AttrValue,
  #[prop_or(AttrValue::Static("Publish a new Offer"))]
  pub new_publish_offer_label: AttrValue,
  pub onshow: Callback<String>,
}

#[component]
pub fn MyPublishedOffersPage(props: &MyPublishedOffersPageProps) -> Html {
  let refresh = use_state(|| 0usize);

  let onclick = use_callback(
    props.on_new_contract_definition.clone(),
    |_, on_new_contract_definition| {
      on_new_contract_definition.emit(());
    },
  );

  let tag_line = props
    .tag_line
    .as_ref()
    .map(|tag_line| html!(<p>{ tag_line }</p>))
    .unwrap_or_default();

  html!(
    <Stack gutter=true>
      <StackItem>
        <Split gutter=true>
          <SplitItem fill=true>
            <Title level={Level::H3} size={Size::XXLarge}>{ &props.title }</Title>
            { tag_line }
          </SplitItem>
          <SplitItem>
            <Button icon={Icon::Plus} {onclick} variant={ButtonVariant::Primary}>
              { &props.new_publish_offer_label }
            </Button>
          </SplitItem>
        </Split>
      </StackItem>
      <StackItem>
        <Suspense>
          <MyPublishedOffersPageInner
            force_refresh={*refresh}
            button_label={props.button_label.clone()}
            onshow={props.onshow.clone()}
          />
        </Suspense>
      </StackItem>
    </Stack>
  )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct MyPublishedOffersPageInnerProps {
  pub force_refresh: usize,
  #[prop_or(AttrValue::Static("Show Offers"))]
  pub button_label: AttrValue,
  pub onshow: Callback<String>,
}

#[component]
pub fn MyPublishedOffersPageInner(props: &MyPublishedOffersPageInnerProps) -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();

  let asset_items = use_future_with(
    (edc_connector_context, props.force_refresh),
    |parameters| async move {
      let (edc_connector_context, _) = (*parameters).clone();

      let query = Query::builder().limit(1000).build();

      if let Some(client) = edc_connector_context.get_client() {
        let asset_ids: HashSet<String> = HashSet::from_iter(
          client
            .contract_definitions(EdcConnectorApiVersion::V4)
            .query(query)
            .await
            .unwrap_or_default()
            .into_iter()
            .flat_map(|contract_definition| {
              contract_definition
                .assets_selector()
                .iter()
                .flat_map(|criterion| match &criterion.operand_right().0 {
                  serde_json::Value::String(identifier) => vec![identifier.to_string()],
                  serde_json::Value::Array(identifiers) => identifiers
                    .iter()
                    .map(|identifier| identifier.as_str().unwrap_or_default().to_string())
                    .collect(),
                  _ => vec![],
                })
                .collect::<Vec<_>>()
            }),
        );

        let mut asset_items = vec![];

        for asset_id in asset_ids {
          if let Ok(asset_item) = client
            .assets(EdcConnectorApiVersion::V4)
            .get(&asset_id)
            .await
            .map(AssetItem::from)
          {
            asset_items.push(asset_item);
          }
        }

        asset_items
      } else {
        vec![]
      }
    },
  )?;

  let asset_items: Vec<_> = (*asset_items).clone();

  Ok(html!(
    <ListAssetsGallery
      {asset_items}
      onshow={props.onshow.clone()}
      button_label={props.button_label.clone()}
    />
  ))
}
