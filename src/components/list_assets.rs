use crate::models::AssetItem;
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
  pub ondelete: Callback<String>,
}

#[component]
pub fn ListAssets(props: &ListAssetsProps) -> Html {
  let header = html_nested! {
    <TableHeader<Columns>>
      <TableColumn<Columns> label="Name" index={Columns::Name} />
      <TableColumn<Columns> label="ID" index={Columns::Id} />
      <TableColumn<Columns> label="Base URL" index={Columns::BaseUrl} />
      <TableColumn<Columns> label="Proxy Path" index={Columns::ProxyPath} />
      <TableColumn<Columns> label="Proxy Query Parameters" index={Columns::ProxyQueryParameters} />
      <TableColumn<Columns> label="Proxy Method" index={Columns::ProxyMethod} />
      <TableColumn<Columns> label="Proxy Body" index={Columns::ProxyBody} />
      <TableColumn<Columns> label="" index={Columns::Actions} />
    </TableHeader<Columns>>
  };

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

  let rows = props
    .asset_items
    .iter()
    .map(|asset_item| AssetRenderer {
      asset_item: asset_item.clone(),
      ondelete: props.ondelete.clone(),
    })
    .collect();

  let (entries, _) = use_table_data(MemoizedTableModel::new(Rc::new(rows)));

  html!(
    <>
      <Toolbar>
        <ToolbarContent>
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
      <Table<Columns, UseTableData<Columns, MemoizedTableModel<AssetRenderer>>>
        mode={TableMode::Compact}
        {header}
        {entries}
      />
    </>
  )
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Columns {
  Id,
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
  ondelete: Callback<String>,
}

impl TableEntryRenderer<Columns> for AssetRenderer {
  fn render_cell(&self, context: CellContext<'_, Columns>) -> Cell {
    match context.column {
      Columns::Id => html! { self.asset_item.id.to_owned() },
      Columns::Name => html!(self.asset_item.name.to_owned()),
      Columns::BaseUrl => html!(self.asset_item.base_url.to_owned()),
      Columns::ProxyPath => html!(self.asset_item.proxy_path),
      Columns::ProxyQueryParameters => html!(self.asset_item.proxy_query_params),
      Columns::ProxyMethod => html!(self.asset_item.proxy_method),
      Columns::ProxyBody => html!(self.asset_item.proxy_body),
      Columns::Actions => {
        let asset_id = self.asset_item.id.to_string();

        html!(<DeleteAsset {asset_id} ondelete={self.ondelete.clone()} />)
      }
    }
    .into()
  }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub asset_id: String,
  pub ondelete: Callback<String>,
}

#[function_component]
pub fn DeleteAsset(props: &Props) -> Html {
  let onclick = use_callback(
    (props.asset_id.clone(), props.ondelete.clone()),
    |_, (asset_id, ondelete)| {
      ondelete.emit(asset_id.to_string());
    },
  );

  html!(<Button variant={ButtonVariant::Danger} icon={Icon::Trash} {onclick}>{ "Delete" }</Button>)
}
