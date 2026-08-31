use crate::components::ListContractAgreements;
use crate::contexts::use_edc_connector_context;
use crate::models::ContractAgreementItem;
use edc_connector_client::EdcConnectorApiVersion;
use edc_connector_client::types::query::{Query, SortOrder};
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ContractAgreementPageProps {
  #[prop_or_default]
  pub onshow: Callback<String>,
}

#[component]
pub fn ContractAgreementPage(props: &ContractAgreementPageProps) -> Html {
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
        <Title level={Level::H3} size={Size::XXLarge}>{ "List Contract Agreements" }</Title>
        <p>{ "A contract agreement is the finalized, legally binding contract signed between parties after successful negotiation." }</p>
      </StackItem>
      <StackItem>
        <Suspense>
          <ContractAgreementPageInner
            offset={*offset}
            limit={*limit}
            {onoffset}
            {onlimit}
            force_refresh={*refresh}
            onshow={props.onshow.clone()}
          />
        </Suspense>
      </StackItem>
    </Stack>
  )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ContractAgreementPageInnerProps {
  pub offset: usize,
  pub limit: usize,
  pub onoffset: Callback<usize>,
  pub onlimit: Callback<usize>,
  pub force_refresh: usize,
  pub onshow: Callback<String>,
}

#[component]
pub fn ContractAgreementPageInner(props: &ContractAgreementPageInnerProps) -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();

  let contract_agreement_items = use_future_with(
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
        .sort("contractSigningDate", SortOrder::Desc)
        .build();

      if let Some(client) = edc_connector_context.get_client() {
        client
          .contract_agreements(EdcConnectorApiVersion::V4)
          .query(query)
          .await
          .unwrap_or_default()
          .into_iter()
          .map(ContractAgreementItem::from)
          .collect::<Vec<_>>()
      } else {
        vec![]
      }
    },
  )?;

  let contract_agreement_items = (*contract_agreement_items).clone();

  Ok(html!(
    <ListContractAgreements
      contract_agreement_items={contract_agreement_items}
      offset={props.offset}
      limit={props.limit}
      onoffset={props.onoffset.clone()}
      onlimit={props.onlimit.clone()}
      onshow={props.onshow.clone()}
    />
  ))
}
