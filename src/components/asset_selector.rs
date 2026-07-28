use crate::contexts::use_edc_connector_context;
use crate::models::AssetItem;
use edc_connector_client::EdcConnectorApiVersion;
use edc_connector_client::types::query::Query;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct AssetSelectorProps {
  pub selected_assets: Vec<AssetItem>,
  pub onselect: Callback<Vec<AssetItem>>,
  #[prop_or("selectable-asset".to_string())]
  pub select_id: String,
}

#[component]
pub fn AssetSelector(props: &AssetSelectorProps) -> Html {
  html!(
    <Suspense>
      <AssetSelectorInner
        onselect={props.onselect.clone()}
        selected_assets={props.selected_assets.clone()}
        select_id={props.select_id.clone()}
      />
    </Suspense>
  )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct AssetSelectorInnerProps {
  pub selected_assets: Vec<AssetItem>,
  pub onselect: Callback<Vec<AssetItem>>,
  pub select_id: String,
}

#[component]
fn AssetSelectorInner(props: &AssetSelectorInnerProps) -> HtmlResult {
  let limit = use_state(|| 10usize);
  let offset = use_state(|| 0usize);
  let force_refresh = use_state(|| 0usize);
  let edc_connector_context = use_edc_connector_context();

  let assets = use_future_with(
    (edc_connector_context, *limit, *offset, *force_refresh),
    |parameters| async move {
      let (edc_connector_context, limit, offset, _) = (*parameters).clone();

      let query = Query::builder()
        .limit(limit as u32)
        .offset(offset as u32)
        .build();

      if let Some(client) = edc_connector_context.get_client() {
        client
          .assets(EdcConnectorApiVersion::V4)
          .query(query)
          .await
          .unwrap_or_default()
          .into_iter()
          .map(AssetItem::from)
          .collect::<Vec<_>>()
      } else {
        vec![]
      }
    },
  )?;

  let assets = (*assets).clone();

  let items = assets.iter().map(|asset_item| {
    let asset_item_name = asset_item.name.to_string();
    let onselect = props.onselect.clone();

    let selectable_actions = {
      let asset_item = asset_item.clone();
      let selected_assets = props.selected_assets.clone();
      let checked = if props.selected_assets.contains(&asset_item) {
        CheckboxState::Checked
      } else {
        CheckboxState::Unchecked
      };

      yew::props!(CardSelectableActionsObjectProperties {
        action: CardSelectableActionsVariant::MultiSelect {
          checked,
          onchange: onselect.reform(move |checked: CheckboxState| {
            let mut selected_assets = selected_assets.clone();

            match checked {
              CheckboxState::Checked => {
                selected_assets.push(asset_item.clone());
              }
              CheckboxState::Unchecked => {
                selected_assets.retain(|item| item.id != asset_item.id);
              }
              CheckboxState::Indeterminate => {}
            }

            selected_assets
          }),
        }
      })
    };

    html!(
      <Card selectable=true selected={props.selected_assets.contains(asset_item)}>
        <CardHeader {selectable_actions}>{ asset_item_name }</CardHeader>
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
