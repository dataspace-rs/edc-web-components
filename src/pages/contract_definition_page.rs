use crate::components::ListContractDefinitions;
use crate::contexts::use_edc_connector_context;
use crate::models::ContractDefinitionItem;
use edc_connector_client::EdcConnectorApiVersion;
use edc_connector_client::types::query::Query;
use patternfly_yew::prelude::*;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ContractDefinitionPageProps {
  pub on_new_contract_definition: Callback<()>,
  #[prop_or_default]
  pub on_policy_click: Callback<String>,
  #[prop_or_default]
  pub on_asset_click: Callback<String>,
}

#[component]
pub fn ContractDefinitionPage(props: &ContractDefinitionPageProps) -> Html {
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
    |contract_definition_id: String, (refresh, edc_connector_context)| {
      let refresh = refresh.clone();
      let edc_connector_context = edc_connector_context.clone();
      let contract_definition_id = contract_definition_id.clone();

      spawn_local(async move {
        if let Some(client) = edc_connector_context.get_client() {
          let _ = client
            .contract_definitions(EdcConnectorApiVersion::V4)
            .delete(&contract_definition_id)
            .await;
        }
        refresh.set(*refresh + 1);
      });
    },
  );

  let onclick = use_callback(
    props.on_new_contract_definition.clone(),
    |_, on_new_contract_definition| {
      on_new_contract_definition.emit(());
    },
  );

  html!(
    <Stack gutter=true>
      <StackItem>
        <Split gutter=true>
          <SplitItem fill=true>
            <Title level={Level::H3} size={Size::XXLarge}>{ "List Contract Definitions" }</Title>
            <p>
              { "A contract definition links an asset to an access policy and a contract policy. The contract definition constitutes an offer to the other participants." }
            </p>
          </SplitItem>
          <SplitItem>
            <Button icon={Icon::Plus} {onclick} variant={ButtonVariant::Primary}>
              { "Create a Contract Definition" }
            </Button>
          </SplitItem>
        </Split>
      </StackItem>
      <StackItem>
        <Suspense>
          <ContractDefinitionPageInner
            offset={*offset}
            limit={*limit}
            {onoffset}
            {onlimit}
            {ondelete}
            on_policy_click={props.on_policy_click.clone()}
            on_asset_click={props.on_asset_click.clone()}
            force_refresh={*refresh}
          />
        </Suspense>
      </StackItem>
    </Stack>
  )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ContractDefinitionPageInnerProps {
  pub offset: usize,
  pub limit: usize,
  pub onoffset: Callback<usize>,
  pub onlimit: Callback<usize>,
  pub ondelete: Callback<String>,
  #[prop_or_default]
  pub on_policy_click: Callback<String>,
  #[prop_or_default]
  pub on_asset_click: Callback<String>,
  pub force_refresh: usize,
}

#[component]
pub fn ContractDefinitionPageInner(props: &ContractDefinitionPageInnerProps) -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();

  let contract_definition_items = use_future_with(
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
        client
          .contract_definitions(EdcConnectorApiVersion::V4)
          .query(query)
          .await
          .unwrap_or_default()
          .into_iter()
          .map(ContractDefinitionItem::from)
          .collect()
      } else {
        vec![]
      }
    },
  )?;

  let contract_definition_items = (*contract_definition_items).clone();

  Ok(html!(
    <ListContractDefinitions
      contract_definition_items={contract_definition_items}
      offset={props.offset}
      limit={props.limit}
      onoffset={props.onoffset.clone()}
      onlimit={props.onlimit.clone()}
      ondelete={props.ondelete.clone()}
      on_policy_click={props.on_policy_click.clone()}
      on_asset_click={props.on_asset_click.clone()}
    />
  ))
}
