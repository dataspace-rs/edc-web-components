use crate::components::DatasetCard;
use crate::models::{AssetItem, DataspaceDataset};
use patternfly_yew::prelude::*;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ListAssetsProps {
  pub asset_items: Vec<AssetItem>,
  pub offset: usize,
  pub limit: usize,
  pub onoffset: Callback<usize>,
  pub onlimit: Callback<usize>,
  pub onshow: Callback<String>,
  #[prop_or(AttrValue::Static("Select"))]
  pub button_label: AttrValue,
}

#[component]
pub fn ListAssets(props: &ListAssetsProps) -> Html {
  let display_cards = use_state(|| true);
  let total_entries: Option<usize> = None;

  let nav_callback = use_callback(
    (
      props.offset,
      props.limit,
      total_entries,
      props.onoffset.clone(),
    ),
    |page: Navigation, (offset, limit, total_entries, onoffset)| {
      let offset = match page {
        Navigation::First => 0,
        Navigation::Last => (total_entries.unwrap_or_default().saturating_sub(1) / limit) * limit,
        Navigation::Previous => *offset - limit,
        Navigation::Next => *offset + limit,
        Navigation::Page(n) => n * limit,
      };

      onoffset.emit(offset);
    },
  );

  let set_display_cards = use_callback(display_cards.setter(), |value, display_cards_setter| {
    display_cards_setter.set(value);
  });

  let items_render = if *display_cards {
    html!(
      <ListAssetsGallery
        asset_items={props.asset_items.clone()}
        onshow={props.onshow.clone()}
        button_label={props.button_label.clone()}
      />
    )
  } else {
    html!(
      <ListAssetsTable
        asset_items={props.asset_items.clone()}
        onshow={props.onshow.clone()}
        button_label={props.button_label.clone()}
      />
    )
  };

  html!(
    <>
      <Toolbar>
        <ToolbarContent>
          <ToolbarItem>
            <Button
              variant={ButtonVariant::Tertiary}
              icon={Icon::ThLarge}
              disabled={*display_cards}
              onclick={set_display_cards.reform(|_| true)}
            />
            <Button
              variant={ButtonVariant::Tertiary}
              icon={Icon::List}
              disabled={!*display_cards}
              onclick={set_display_cards.reform(|_| false)}
            />
          </ToolbarItem>
          <ToolbarItem r#type={ToolbarItemType::Pagination}>
            <Pagination
              offset={props.offset}
              entries_per_page_choices={vec![5, 10, 25, 50, 100]}
              selected_choice={props.limit}
              onlimit={&props.onlimit}
              onnavigation={&nav_callback}
            />
          </ToolbarItem>
        </ToolbarContent>
      </Toolbar>
      { items_render }
    </>
  )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ListAssetsTableProps {
  pub asset_items: Vec<AssetItem>,
  pub onshow: Callback<String>,
  #[prop_or(AttrValue::Static("Select"))]
  pub button_label: AttrValue,
}

#[component]
pub fn ListAssetsGallery(props: &ListAssetsTableProps) -> Html {
  let rows = props
    .asset_items
    .clone()
    .into_iter()
    .map(DataspaceDataset::from)
    .map(|dataset| {
      let asset_id = dataset.id.clone();

      html!(
        <DatasetCard
          {dataset}
          on_offer_click={props.onshow.reform(move |_| asset_id.clone())}
          button_label={props.button_label.clone()}
        />
      )
    });

  html!(<Gallery gutter=true>{ for rows }</Gallery>)
}

#[component]
pub fn ListAssetsTable(props: &ListAssetsTableProps) -> Html {
  let header = html_nested! {
    <TableHeader<Columns>>
      <TableColumn<Columns> label="Name" index={Columns::Name} />
      <TableColumn<Columns> label="Base URL" index={Columns::BaseUrl} />
      <TableColumn<Columns> label="Proxy Path" index={Columns::ProxyPath} />
      <TableColumn<Columns> label="Proxy Query Parameters" index={Columns::ProxyQueryParameters} />
      <TableColumn<Columns> label="Proxy Method" index={Columns::ProxyMethod} />
      <TableColumn<Columns> label="Proxy Body" index={Columns::ProxyBody} />
      <TableColumn<Columns> label="" index={Columns::Actions} />
    </TableHeader<Columns>>
  };

  let rows = props
    .asset_items
    .iter()
    .map(|asset_item| AssetRenderer {
      asset_item: asset_item.clone(),
      onshow: props.onshow.clone(),
    })
    .collect();

  let (entries, _) = use_table_data(MemoizedTableModel::new(Rc::new(rows)));

  html!(
    <Table<Columns, UseTableData<Columns, MemoizedTableModel<AssetRenderer>>>
      mode={TableMode::Compact}
      {header}
      {entries}
    />
  )
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Columns {
  Name,
  BaseUrl,
  ProxyPath,
  ProxyQueryParameters,
  ProxyMethod,
  ProxyBody,
  Actions,
}

#[derive(Clone, Debug)]
struct AssetRenderer {
  asset_item: AssetItem,
  onshow: Callback<String>,
}

impl TableEntryRenderer<Columns> for AssetRenderer {
  fn render_cell(&self, context: CellContext<'_, Columns>) -> Cell {
    match context.column {
      Columns::Name => html!(self.asset_item.name.to_owned()),
      Columns::BaseUrl => html!(self.asset_item.base_url.to_owned()),
      Columns::ProxyPath => html!(self.asset_item.proxy_path),
      Columns::ProxyQueryParameters => html!(self.asset_item.proxy_query_params),
      Columns::ProxyMethod => html!(self.asset_item.proxy_method),
      Columns::ProxyBody => html!(self.asset_item.proxy_body),
      Columns::Actions => {
        let asset_id = self.asset_item.id.to_string();

        html!(<ShowAsset {asset_id} onshow={self.onshow.clone()} />)
      }
    }
    .into()
  }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ShowAssetProps {
  pub asset_id: String,
  pub onshow: Callback<String>,
}

#[function_component]
pub fn ShowAsset(props: &ShowAssetProps) -> Html {
  let onclick = use_callback(
    (props.asset_id.clone(), props.onshow.clone()),
    |_, (asset_id, onshow)| {
      onshow.emit(asset_id.to_string());
    },
  );

  html!(<Button variant={ButtonVariant::Primary} icon={Icon::Eye} {onclick}>{ "Show" }</Button>)
}
