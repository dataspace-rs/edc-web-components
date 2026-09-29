use crate::components::ListCommonExpressionLanguage;
use crate::contexts::{
  CelExpressionLibraryItem, use_cel_expression_library_context, use_edc_connector_context,
};
use crate::models::CommonExpressionLanguageItem;
use edc_connector_client::EdcConnectorApiVersion;
use edc_connector_client::types::common_expression_language::NewCommonExpressionLanguage;
use edc_connector_client::types::query::Query;
use log::error;
use patternfly_yew::prelude::*;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CommonExpressionLanguagePageProps {
  pub on_new_cel: Callback<()>,
  #[prop_or("Common Expression Language Library".to_string())]
  pub title: String,
  #[prop_or(None)]
  pub tag_line: Option<String>,
  #[prop_or("Create a CEL Expression".to_string())]
  pub create_title: String,
  #[prop_or("Show Library".to_string())]
  pub open_library_title: String,
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
    (refresh.clone(), edc_connector_context.clone()),
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

  let tag_line = props
    .tag_line
    .as_ref()
    .map(|tag_line| html!(<p>{ tag_line }</p>))
    .unwrap_or_default();

  let cel_expression_library_context = use_cel_expression_library_context();

  let expanded = use_state_eq(|| false);
  let onclick = use_callback(expanded.clone(), |_, expanded| {
    expanded.set(!**expanded);
  });

  let cel_expression_library = cel_expression_library_context.as_ref().map(|_| {
    html_nested!(
      <SplitItem>
        <Button icon={Icon::Catalog} {onclick} variant={ButtonVariant::Secondary}>
          { &props.open_library_title }
        </Button>
      </SplitItem>
    )
  });

  let import_cel_expression = use_callback(
    (edc_connector_context.clone(), refresh.clone()),
    |cel_expression_library_item: CelExpressionLibraryItem, (edc_connector_context, refresh)| {
      let edc_connector_context = edc_connector_context.clone();
      let refresh = refresh.clone();

      let new_cel_builder = NewCommonExpressionLanguage::builder()
        .left_operand(cel_expression_library_item.left_operand.to_string())
        .description(cel_expression_library_item.description.unwrap_or_default())
        .scopes(cel_expression_library_item.scopes)
        .expression(cel_expression_library_item.expression);

      let new_cel = new_cel_builder.build();

      spawn_local(async move {
        if let Some(client) = edc_connector_context.get_client() {
          match client
            .common_expression_language(EdcConnectorApiVersion::V5Beta)
            .create(&new_cel)
            .await
          {
            Ok(_) => {
              refresh.set(*refresh + 1);
            }
            Err(error) => {
              error!("Failed to import CEL expression: {}", error);
            }
          }
        }
      })
    },
  );

  let panel_content = cel_expression_library_context.clone().map(move |cel_expression_library| {
    let items =
      cel_expression_library.cel_expression_library().iter().map(|item| {
        let name = item.name.to_string();
        let item = item.clone();

        html!(
          <StackItem>
            <Button
              variant={ButtonVariant::Control}
              icon={Icon::Import}
              onclick={import_cel_expression.reform(move |_| item.clone())}
            >
              { name }
            </Button>
          </StackItem>
        )
      });

    html!(
      <Panel>
        <PanelHeader>{ "CEL Expression Library" }</PanelHeader>
        <PanelMain>
          <PanelMainBody>
            <Stack gutter=true>{ for items }</Stack>
          </PanelMainBody>
        </PanelMain>
      </Panel>
    )
  });

  html!(
    <Stack gutter=true>
      <StackItem>
        <Split gutter=true>
          <SplitItem fill=true>
            <Title level={Level::H3} size={Size::XXLarge}>{ &props.title }</Title>
            { tag_line }
          </SplitItem>
          { cel_expression_library }
          <SplitItem>
            <Button
              icon={Icon::Plus}
              onclick={props.on_new_cel.reform(|_| ())}
              variant={ButtonVariant::Primary}
            >
              { &props.create_title }
            </Button>
          </SplitItem>
        </Split>
      </StackItem>
      <StackItem>
        <Drawer expanded={*expanded} inline=true>
          <DrawerContent {panel_content}>
            <DrawerContentBody>
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
            </DrawerContentBody>
          </DrawerContent>
        </Drawer>
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
