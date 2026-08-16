use crate::components::ListContractNegotiations;
use crate::contexts::use_edc_connector_context;
use crate::models::ContractNegotiationItem;
use edc_connector_client::EdcConnectorApiVersion;
use edc_connector_client::types::query::Query;
use patternfly_yew::prelude::*;
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
  let switch = use_state(|| false);

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

  let onswitch = use_callback(
    (refresh.clone(), switch.setter()),
    |switch, (refresh, switch_setter)| {
      switch_setter.set(switch);
      refresh.set(**refresh + 1);
    },
  );

  let fallback = html! {<Bullseye><Spinner size={SpinnerSize::Lg} /></Bullseye>};

  html!(
    <Stack gutter=true>
      <StackItem>
        <Split gutter=true>
          <SplitItem fill=true>
            <Title level={Level::H3} size={Size::XXLarge}>{ "List Contract Negotiations" }</Title>
          </SplitItem>
        </Split>
      </StackItem>
      <StackItem>
        <Suspense {fallback}>
          <ContractNegotiationPageInner
            offset={*offset}
            limit={*limit}
            switch={*switch}
            {onoffset}
            {onlimit}
            {onswitch}
            force_refresh={*refresh}
            on_show_contract_negotiation={props.on_show_contract_negotiation.clone()}
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
  pub switch: bool,
  pub onoffset: Callback<usize>,
  pub onlimit: Callback<usize>,
  pub onswitch: Callback<bool>,
  pub force_refresh: usize,
  pub on_show_contract_negotiation: Callback<String>,
}

#[component]
pub fn ContractNegotiationPageInner(props: &ContractNegotiationPageInnerProps) -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();

  let contract_negotiation_items = use_future_with(
    (
      edc_connector_context,
      props.limit,
      props.offset,
      props.switch,
      props.force_refresh,
    ),
    |parameters| async move {
      let (edc_connector_context, limit, offset, switch, _) = (*parameters).clone();

      let query = Query::builder()
        .limit(limit as u32)
        .offset(offset as u32)
        .filter("type", "=", if switch == false {"PROVIDER"} else {"CONSUMER"} )
        .build();

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
      switch={props.switch}
      onoffset={props.onoffset.clone()}
      onlimit={props.onlimit.clone()}
      onswitch={props.onswitch.clone()}
      on_show_contract_negotiation={props.on_show_contract_negotiation.clone()}
    />
  ))
}
