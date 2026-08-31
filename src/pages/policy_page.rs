use crate::components::ListPolicies;
use crate::contexts::use_edc_connector_context;
use crate::models::PolicyDefinitionItem;
use edc_connector_client::EdcConnectorApiVersion;
use edc_connector_client::types::query::Query;
use patternfly_yew::prelude::*;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PolicyPageProps {
  pub on_new_policy: Callback<()>,
  pub onshow: Callback<String>,
}

#[component]
pub fn PolicyPage(props: &PolicyPageProps) -> Html {
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
    |policy_id: String, (refresh, edc_connector_context)| {
      let refresh = refresh.clone();
      let edc_connector_context = edc_connector_context.clone();
      let policy_id = policy_id.clone();

      spawn_local(async move {
        if let Some(client) = edc_connector_context.get_client() {
          let _ = client
            .policies(EdcConnectorApiVersion::V4)
            .delete(&policy_id)
            .await;
          log::warn!("Deleted policy {} - {}", policy_id, *refresh + 1);
          refresh.set(*refresh + 1);
        }
      });
    },
  );

  let onclick = use_callback(props.on_new_policy.clone(), |_, on_new_policy| {
    on_new_policy.emit(());
  });

  html!(
    <Stack gutter=true>
      <StackItem>
        <Split gutter=true>
          <SplitItem fill=true>
            <Title level={Level::H3} size={Size::XXLarge}>{ "List Policies" }</Title>
            <p>
              { "A policy consists of one or more rules. The policy is then bound to an asset during the creation of the contract definition." }
            </p>
          </SplitItem>
          <SplitItem>
            <Button icon={Icon::Plus} {onclick} variant={ButtonVariant::Primary}>
              { "Create a Policy" }
            </Button>
          </SplitItem>
        </Split>
      </StackItem>
      <StackItem>
        <Suspense>
          <PolicyPageInner
            offset={*offset}
            limit={*limit}
            {onoffset}
            {onlimit}
            {ondelete}
            onshow={props.onshow.clone()}
            force_refresh={*refresh}
          />
        </Suspense>
      </StackItem>
    </Stack>
  )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PolicyPageInnerProps {
  pub offset: usize,
  pub limit: usize,
  pub onoffset: Callback<usize>,
  pub onlimit: Callback<usize>,
  pub ondelete: Callback<String>,
  pub onshow: Callback<String>,
  pub force_refresh: usize,
}

#[component]
pub fn PolicyPageInner(props: &PolicyPageInnerProps) -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();

  let policy_definition_items = use_future_with(
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
          .policies(EdcConnectorApiVersion::V4)
          .query(query)
          .await
          .unwrap_or_default()
          .into_iter()
          .map(PolicyDefinitionItem::from)
          .collect::<Vec<_>>()
      } else {
        vec![]
      }
    },
  )?;

  let policy_definition_items = (*policy_definition_items).clone();

  Ok(html!(
    <ListPolicies
      policy_definition_items={policy_definition_items}
      offset={props.offset}
      limit={props.limit}
      onoffset={props.onoffset.clone()}
      onlimit={props.onlimit.clone()}
      ondelete={props.ondelete.clone()}
      onshow={props.onshow.clone()}
    />
  ))
}
