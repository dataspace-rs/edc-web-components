use crate::components::ListAssets;
use crate::contexts::use_edc_connector_context;
use crate::models::AssetItem;
use edc_connector_client::EdcConnectorApiVersion;
use edc_connector_client::types::query::Query;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct AssetPageProps {
  pub on_new_asset: Callback<()>,
  pub onshow: Callback<String>,
}

#[component]
pub fn AssetPage(props: &AssetPageProps) -> Html {
  let refresh = use_state(|| 0usize);
  let offset = use_state(|| 0usize);
  let limit = use_state(|| 10usize);

  let onoffset = use_callback(
    (refresh.clone(), offset.setter()),
    |offset, (refresh, offset_setter)| {
      offset_setter.set(offset);
      refresh.set(**refresh + 1);
    },
  );

  let onlimit = use_callback(
    (refresh.clone(), limit.setter()),
    |limit, (refresh, limit_setter)| {
      limit_setter.set(limit);
      refresh.set(**refresh + 1);
    },
  );

  let onclick = use_callback(props.on_new_asset.clone(), |_, on_new_asset| {
    on_new_asset.emit(());
  });

  html!(
    <Stack gutter=true>
      <StackItem>
        <Split gutter=true>
          <SplitItem fill=true>
            <Title level={Level::H3} size={Size::XXLarge}>{ "List Assets" }</Title>
          </SplitItem>
          <SplitItem>
            <Button icon={Icon::Plus} {onclick} variant={ButtonVariant::Primary}>
              { "Create an Asset" }
            </Button>
          </SplitItem>
        </Split>
      </StackItem>
      <StackItem>
        <Suspense>
          <AssetPageInner
            offset={*offset}
            limit={*limit}
            {onoffset}
            {onlimit}
            onshow={props.onshow.clone()}
            force_refresh={*refresh}
          />
        </Suspense>
      </StackItem>
    </Stack>
  )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct AssetPageInnerProps {
  pub offset: usize,
  pub limit: usize,
  pub onoffset: Callback<usize>,
  pub onlimit: Callback<usize>,
  pub onshow: Callback<String>,
  pub force_refresh: usize,
}

#[component]
pub fn AssetPageInner(props: &AssetPageInnerProps) -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();

  let asset_items = use_future_with(
    (
      edc_connector_context,
      props.limit,
      props.offset,
      props.force_refresh,
    ),
    |parameters| async move {
      let (edc_connector_context, limit, offset, _) = (*parameters).clone();

      let query = Query::builder()
        .limit(limit as u32)
        .offset(offset as u32)
        .build();

      if let Some(client) = edc_connector_context.get_client() {
        let items = client
          .assets(EdcConnectorApiVersion::V4)
          .query(query)
          .await
          .map_err(|error| {
            log::error!("Error: {}", error);
            error
          })
          .unwrap_or_default();

        items.into_iter().map(AssetItem::from).collect::<Vec<_>>()
      } else {
        vec![]
      }
    },
  )?;

  let asset_items = (*asset_items).clone();

  Ok(html!(
    <ListAssets
      asset_items={asset_items}
      offset={props.offset}
      limit={props.limit}
      onoffset={props.onoffset.clone()}
      onlimit={props.onlimit.clone()}
      onshow={props.onshow.clone()}
      button_label="Show"
    />
  ))
}
