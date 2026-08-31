use crate::components::ListTransferProcesses;
use crate::contexts::use_edc_connector_context;
use crate::models::TransferProcessItem;
use edc_connector_client::types::query::SortOrder;
use edc_connector_client::{EdcConnectorApiVersion, types::query::Query};
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TransferProcessPageProps {
  #[prop_or_default]
  pub contract_agreement_id: Option<String>,
  #[prop_or_default]
  pub onshow: Callback<String>,
}

#[component]
pub fn TransferProcessPage(props: &TransferProcessPageProps) -> Html {
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

  html!(
    <Stack gutter=true>
      <StackItem>
        <Title level={Level::H3} size={Size::XXLarge}>{ "List Transfer Processes" }</Title>
      </StackItem>
      <StackItem>
        <Suspense>
          <TransferProcessPageInner
            offset={*offset}
            limit={*limit}
            {onoffset}
            {onlimit}
            contract_agreement_id={props.contract_agreement_id.clone()}
            force_refresh={*refresh}
            onshow={props.onshow.clone()}
          />
        </Suspense>
      </StackItem>
    </Stack>
  )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TransferProcessPageInnerProps {
  pub offset: usize,
  pub limit: usize,
  pub onoffset: Callback<usize>,
  pub onlimit: Callback<usize>,
  pub contract_agreement_id: Option<String>,
  pub force_refresh: usize,
  pub onshow: Callback<String>,
}

#[component]
pub fn TransferProcessPageInner(props: &TransferProcessPageInnerProps) -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();

  let transfer_processe_items = use_future_with(
    (
      edc_connector_context,
      props.limit,
      props.offset,
      props.contract_agreement_id.clone(),
      props.force_refresh,
    ),
    |parameters| async move {
      let (edc_connector_context, limit, offset, contract_agreement_id, _) = (*parameters).clone();

      let query_builder = Query::builder().limit(limit as u32).offset(offset as u32);

      let query_builder = if let Some(contract_agreement_id) = contract_agreement_id {
        query_builder.filter("contractId", "=", contract_agreement_id)
      } else {
        query_builder
      };

      let query = query_builder
        .sort("stateTimestamp", SortOrder::Desc)
        .build();

      if let Some(client) = edc_connector_context.get_client() {
        client
          .transfer_processes(EdcConnectorApiVersion::V4)
          .query(query)
          .await
          .unwrap_or_default()
          .into_iter()
          .map(TransferProcessItem::from)
          .collect::<Vec<_>>()
      } else {
        vec![]
      }
    },
  )?;

  let transfer_processe_items = (*transfer_processe_items).clone();

  Ok(html!(
    <ListTransferProcesses
      transfer_processe_items={transfer_processe_items}
      offset={props.offset}
      limit={props.limit}
      onoffset={props.onoffset.clone()}
      onlimit={props.onlimit.clone()}
      onshow={props.onshow.clone()}
    />
  ))
}
