use crate::contexts::use_edc_connector_context;
use crate::models::PolicyDefinitionItem;
use edc_connector_client::EdcConnectorApiVersion;
use edc_connector_client::types::query::Query;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PolicySelectorProps {
  pub selected_policy: Option<PolicyDefinitionItem>,
  pub onselect: Callback<PolicyDefinitionItem>,
  #[prop_or("selectable-policy".to_string())]
  pub select_id: String,
}

#[component]
pub fn PolicySelector(props: &PolicySelectorProps) -> Html {
  html!(
    <Suspense>
      <PolicySelectorInner
        onselect={props.onselect.clone()}
        selected_policy={props.selected_policy.clone()}
        select_id={props.select_id.clone()}
      />
    </Suspense>
  )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PolicySelectorInnerProps {
  pub selected_policy: Option<PolicyDefinitionItem>,
  pub onselect: Callback<PolicyDefinitionItem>,
  pub select_id: String,
}

#[component]
fn PolicySelectorInner(props: &PolicySelectorInnerProps) -> HtmlResult {
  let limit = use_state(|| 10usize);
  let offset = use_state(|| 0usize);
  let force_refresh = use_state(|| 0usize);
  let edc_connector_context = use_edc_connector_context();

  let policies = use_future_with(
    (edc_connector_context, *limit, *offset, *force_refresh),
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

  let policies = (*policies).clone();

  let items = policies.iter().map(|policy_definition_item| {
    let policy_definition_item_id = policy_definition_item.id.to_string();
    let onselect = props.onselect.clone();

    let selectable_actions = {
      let policy_definition_item = policy_definition_item.clone();

      yew::props!(CardSelectableActionsObjectProperties {
        action: CardSelectableActionsVariant::Click {
          onclick: Some(onselect.reform(move |_| policy_definition_item.clone()))
        },
        base: yew::props!(CardSelectableActionsObjectBase {
          name: props.select_id.clone(),
        })
      })
    };

    html!(
      <Card
        selectable=true
        selected={Some(policy_definition_item) == props.selected_policy.as_ref()}
      >
        <CardHeader {selectable_actions}>{ policy_definition_item_id }</CardHeader>
      </Card>
    )
  });

  let onlimit = use_callback(limit.setter(), |limit, limit_setter| {
    limit_setter.set(limit)
  });

  let total_entries = Option::<usize>::None;

  let nav_callback = use_callback(
    (offset.clone(), *limit, total_entries),
    |page: Navigation, (offset, limit, total_entries)| {
      let new_offset = match page {
        Navigation::First => 0,
        Navigation::Last => (total_entries.unwrap_or_default().saturating_sub(1) / limit) * limit,
        Navigation::Previous => **offset - limit,
        Navigation::Next => **offset + limit,
        Navigation::Page(n) => n * limit,
      };
      offset.set(new_offset);
    },
  );

  Ok(html!(
    <Stack gutter=true>
      <StackItem>
        <Pagination
          offset={*offset}
          entries_per_page_choices={vec![5, 10, 25, 50, 100]}
          selected_choice={*limit}
          onlimit={&onlimit}
          onnavigation={&nav_callback}
        />
      </StackItem>
      <StackItem>
        <Gallery gutter=true>{ for items }</Gallery>
      </StackItem>
    </Stack>
  ))
}
