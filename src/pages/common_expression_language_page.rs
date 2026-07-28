use crate::components::ListCommonExpressionLanguage;
use crate::contexts::use_edc_connector_context;
use crate::models::CommonExpressionLanguageItem;
use edc_connector_client::EdcConnectorApiVersion;
use edc_connector_client::types::query::Query;
use patternfly_yew::prelude::*;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CommonExpressionLanguagePageProps {
  pub on_new_cel: Callback<()>,
}
#[component]
pub fn CommonExpressionLanguagePage(props: &CommonExpressionLanguagePageProps) -> Html {
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

  let edc_connector_context = use_edc_connector_context();

  let ondelete = use_callback(
    (refresh.clone(), edc_connector_context),
    |asset_id: String, (refresh, edc_connector_context)| {
      let refresh = refresh.clone();
      let edc_connector_context = edc_connector_context.clone();
      let asset_id = asset_id.clone();

      spawn_local(async move {
        if let Some(client) = edc_connector_context.get_client() {
          let _ = client
            .common_expression_language(EdcConnectorApiVersion::V5Beta)
            .delete(&asset_id)
            .await;
        }
        refresh.set(*refresh + 1);
      });
    },
  );

  let onclick = use_callback(props.on_new_cel.clone(), |_, on_new_cel| {
    on_new_cel.emit(());
  });

  html!(
    <Stack gutter=true>
      <StackItem>
        <Split gutter=true>
          <SplitItem fill=true>
            <Title level={Level::H3} size={Size::XXLarge}>
              { "Common Expression Language Library" }
            </Title>
          </SplitItem>
          <SplitItem>
            <Button icon={Icon::Plus} {onclick} variant={ButtonVariant::Primary}>
              { "Create an CEL Expression" }
            </Button>
          </SplitItem>
        </Split>
      </StackItem>
      <StackItem>
        <Suspense>
          <CommonExpressionLanguagePageInner
            offset={*offset}
            limit={*limit}
            {onoffset}
            {onlimit}
            {ondelete}
            force_refresh={*refresh}
          />
        </Suspense>
      </StackItem>
    </Stack>
  )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CommonExpressionLanguagePageInnerProps {
  pub offset: usize,
  pub limit: usize,
  pub onoffset: Callback<usize>,
  pub onlimit: Callback<usize>,
  pub ondelete: Callback<String>,
  pub force_refresh: usize,
}

#[component]
pub fn CommonExpressionLanguagePageInner(
  props: &CommonExpressionLanguagePageInnerProps,
) -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();

  let common_expression_language_items = use_future_with(
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
        // .filter("https://w3id.org/edc/v0.0.1/ns/master-catalog-company-id", "=", "424F9F7A-BBC8-4BAD-B128-C3D0A693ABBA")
        .build();

      if let Some(client) = edc_connector_context.get_client() {
        client
          .common_expression_language(EdcConnectorApiVersion::V5Beta)
          .query(query)
          .await
          .map_err(|error| {
            log::error!("Error: {}", error);
            error
          })
          .unwrap_or_default()
          .into_iter()
          .map(CommonExpressionLanguageItem::from)
          .collect::<Vec<_>>()
      } else {
        vec![]
      }
    },
  )?;

  let common_expression_language_items = (*common_expression_language_items).clone();

  Ok(html!(
    <ListCommonExpressionLanguage
      common_expression_language_items={common_expression_language_items}
      offset={props.offset}
      limit={props.limit}
      onoffset={props.onoffset.clone()}
      onlimit={props.onlimit.clone()}
      ondelete={props.ondelete.clone()}
    />
  ))
}
