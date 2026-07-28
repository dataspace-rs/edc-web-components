use crate::models::TransferProcessItem;
use patternfly_yew::prelude::*;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ListTransferProcessesProps {
  pub transfer_processe_items: Vec<TransferProcessItem>,
  pub offset: usize,
  pub limit: usize,
  pub onoffset: Callback<usize>,
  pub onlimit: Callback<usize>,
}

#[component]
pub fn ListTransferProcesses(props: &ListTransferProcessesProps) -> Html {
  let header = html_nested! {
    <TableHeader<Columns>>
      <TableColumn<Columns> label="ID" index={Columns::Id} />
      <TableColumn<Columns> label="State" index={Columns::State} />
      <TableColumn<Columns> label="Asset ID" index={Columns::AssetId} />
      <TableColumn<Columns> label="Contract ID" index={Columns::ContractId} />
      <TableColumn<Columns> label="Transfer Type" index={Columns::TransferType} />
      <TableColumn<Columns> label="Kind" index={Columns::Kind} />
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
    .transfer_processe_items
    .iter()
    .map(|transfer_process_item| ListTransferProcessRenderer(transfer_process_item.clone()))
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
      <Table<Columns, UseTableData<Columns, MemoizedTableModel<ListTransferProcessRenderer>>>
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
  State,
  AssetId,
  ContractId,
  TransferType,
  Kind,
}

#[derive(Clone, Debug)]
struct ListTransferProcessRenderer(TransferProcessItem);

impl TableEntryRenderer<Columns> for ListTransferProcessRenderer {
  fn render_cell(&self, context: CellContext<'_, Columns>) -> Cell {
    match context.column {
      Columns::Id => html! { self.0.id.to_string() },
      Columns::State => html!(self.0.state.to_string()),
      Columns::AssetId => html!(self.0.asset_id.to_string()),
      Columns::ContractId => html! { self.0.contract_id.to_string() },
      Columns::TransferType => html!(self.0.transfer_type.to_string()),
      Columns::Kind => html!(self.0.kind.to_string()),
    }
    .into()
  }
}
