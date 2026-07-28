use crate::models::ContractNegotiationItem;
use patternfly_yew::prelude::*;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ListContractNegotiationsProps {
  pub contract_negotiation_items: Vec<ContractNegotiationItem>,
  pub offset: usize,
  pub limit: usize,
  pub onoffset: Callback<usize>,
  pub onlimit: Callback<usize>,
}

#[component]
pub fn ListContractNegotiations(props: &ListContractNegotiationsProps) -> Html {
  let header = html_nested! {
    <TableHeader<Columns>>
      <TableColumn<Columns> label="ID" index={Columns::Id} />
      <TableColumn<Columns> label="State" index={Columns::State} />
      <TableColumn<Columns> label="Contract Agreement ID" index={Columns::ContractAgreementId} />
      <TableColumn<Columns> label="Counter Party ID" index={Columns::CounterPartyId} />
      <TableColumn<Columns> label="Counter Party Address" index={Columns::CounterPartyAddress} />
      <TableColumn<Columns> label="Protocol" index={Columns::Protocol} />
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
    .contract_negotiation_items
    .iter()
    .map(|contract_negotiation_item| {
      ContractNegotiationItemRenderer(contract_negotiation_item.clone())
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
      <Table<Columns, UseTableData<Columns, MemoizedTableModel<ContractNegotiationItemRenderer>>>
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
  ContractAgreementId,
  CounterPartyId,
  CounterPartyAddress,
  Protocol,
  Kind,
}

#[derive(Clone, Debug)]
struct ContractNegotiationItemRenderer(ContractNegotiationItem);

impl TableEntryRenderer<Columns> for ContractNegotiationItemRenderer {
  fn render_cell(&self, context: CellContext<'_, Columns>) -> Cell {
    match context.column {
      Columns::Id => html! { self.0.id.to_string() },
      Columns::State => html! { self.0.state.to_string() },
      Columns::ContractAgreementId => html! { self.0.contract_agreement_id.to_string() },
      Columns::CounterPartyId => html! { self.0.counter_party_id.to_string() },
      Columns::CounterPartyAddress => html! { self.0.counter_party_address.to_string() },
      Columns::Protocol => html! { self.0.protocol.to_string() },
      Columns::Kind => html! { self.0.kind.to_string() },
    }
    .into()
  }
}
