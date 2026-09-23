use crate::components::ListContractNegotiations;
use crate::contexts::use_edc_connector_context;
use crate::models::{ConsumerProvider, ContractNegotiationItem};
use edc_connector_client::EdcConnectorApiVersion;
use edc_connector_client::types::query::{Query, SortOrder};
use patternfly_yew::prelude::*;
use std::collections::HashSet;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ContractNegotiationPageProps {
  pub on_show_contract_negotiation: Callback<String>,
}

#[component]
pub fn ContractNegotiationPage(props: &ContractNegotiationPageProps) -> Html {
  let refresh = use_state(|| 0usize);
  let offset = use_state(|| 0usize);
  let limit = use_state(|| 10usize);
  let consumer_provider = use_state(HashSet::default);
  let statuses = use_state(|| {
    vec![
      ("Initial".to_string(), false),
      ("Requesting".to_string(), false),
      ("Requested".to_string(), false),
      ("Offering".to_string(), false),
      ("Offered".to_string(), false),
      ("Accepting".to_string(), false),
      ("Accepted".to_string(), false),
      ("Agreeing".to_string(), false),
      ("Agreed".to_string(), false),
      ("Verifying".to_string(), false),
      ("Verified".to_string(), false),
      ("Finalizing".to_string(), false),
      ("Finalized".to_string(), false),
      ("Terminating".to_string(), false),
      ("Terminated".to_string(), false),
    ]
  });

  let on_offset = use_callback(
    (refresh.clone(), offset.setter()),
    |offset, (refresh, offset_setter)| {
      offset_setter.set(offset);
      refresh.set(**refresh + 1);
    },
  );

  let on_limit = use_callback(
    (refresh.clone(), limit.setter()),
    |limit, (refresh, limit_setter)| {
      limit_setter.set(limit);
      refresh.set(**refresh + 1);
    },
  );

  let on_consumer_provider = use_callback(
    (refresh.clone(), consumer_provider.setter()),
    |consumer_provider, (refresh, consumer_provider_setter)| {
      consumer_provider_setter.set(consumer_provider);
      refresh.set(**refresh + 1);
    },
  );

  let on_statuses = use_callback(
    (refresh.clone(), statuses.setter()),
    |statuses, (refresh, statuses_setter)| {
      statuses_setter.set(statuses);
      refresh.set(**refresh + 1);
    },
  );

  let fallback = html! {
    <Bullseye>
      <Spinner size={SpinnerSize::Lg} />
    </Bullseye>
  };

  html!(
    <Stack gutter=true>
      <StackItem>
        <Split gutter=true>
          <SplitItem fill=true>
            <Title level={Level::H3} size={Size::XXLarge}>{ "List Contract Negotiations" }</Title>
            <p>
              { "A contract negotiation represents the active, multi-step process between two parties to agree on data sharing terms." }
            </p>
          </SplitItem>
        </Split>
      </StackItem>
      <StackItem>
        <Suspense {fallback}>
          <ContractNegotiationPageInner
            offset={*offset}
            limit={*limit}
            statuses={(*statuses).clone()}
            {on_offset}
            {on_limit}
            {on_statuses}
            force_refresh={*refresh}
            on_show_contract_negotiation={props.on_show_contract_negotiation.clone()}
            consumer_provider={(*consumer_provider).clone()}
            {on_consumer_provider}
          />
        </Suspense>
      </StackItem>
    </Stack>
  )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ContractNegotiationPageInnerProps {
  pub offset: usize,
  pub limit: usize,
  #[prop_or_default]
  pub statuses: Vec<(String, bool)>,
  pub on_offset: Callback<usize>,
  pub on_limit: Callback<usize>,
  pub on_statuses: Callback<Vec<(String, bool)>>,
  pub force_refresh: usize,
  pub on_show_contract_negotiation: Callback<String>,
  #[prop_or(true)]
  pub show_status_selector: bool,
  #[prop_or_default]
  pub consumer_provider: HashSet<ConsumerProvider>,
  pub on_consumer_provider: Callback<HashSet<ConsumerProvider>>,
}

#[component]
pub fn ContractNegotiationPageInner(props: &ContractNegotiationPageInnerProps) -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();

  let contract_negotiation_items = use_future_with(
    (
      edc_connector_context,
      props.limit,
      props.offset,
      props.consumer_provider.clone(),
      props.statuses.clone(),
      props.force_refresh,
    ),
    |parameters| async move {
      let (edc_connector_context, limit, offset, consumer_provider, statuses, _) =
        (*parameters).clone();

      let query_builder = Query::builder().limit(limit as u32).offset(offset as u32);

      let query_builder = match (
        consumer_provider.contains(&ConsumerProvider::Consumer),
        consumer_provider.contains(&ConsumerProvider::Provider),
      ) {
        (true, true) => query_builder,
        (true, false) => query_builder.filter("type", "=", "CONSUMER"),
        (false, true) => query_builder.filter("type", "=", "PROVIDER"),
        (false, false) => query_builder,
      };

      let query_builder = if statuses.iter().all(|(_, selected)| *selected)
        || statuses.iter().all(|(_, selected)| !*selected)
      {
        query_builder
      } else {
        let list = statuses
          .iter()
          .filter_map(|(label, selected)| {
            if *selected {
              Some(label.clone().to_uppercase())
            } else {
              None
            }
          })
          .collect::<Vec<_>>();

        query_builder.filter("state", "IN", list)
      };

      let query = query_builder.sort("createdAt", SortOrder::Desc).build();

      if let Some(client) = edc_connector_context.get_client() {
        client
          .contract_negotiations(EdcConnectorApiVersion::V4)
          .query(query)
          .await
          .unwrap_or_default()
          .into_iter()
          .map(ContractNegotiationItem::from)
          .collect()
      } else {
        vec![]
      }
    },
  )?;

  let contract_negotiation_items = (*contract_negotiation_items).clone();

  Ok(html!(
    <ListContractNegotiations
      contract_negotiation_items={contract_negotiation_items}
      offset={props.offset}
      limit={props.limit}
      statuses={props.statuses.clone()}
      on_offset={props.on_offset.clone()}
      on_limit={props.on_limit.clone()}
      on_show_contract_negotiation={props.on_show_contract_negotiation.clone()}
      on_statuses={props.on_statuses.clone()}
      show_status_selector={props.show_status_selector}
      consumer_provider={props.consumer_provider.clone()}
      on_consumer_provider={props.on_consumer_provider.clone()}
    />
  ))
}
