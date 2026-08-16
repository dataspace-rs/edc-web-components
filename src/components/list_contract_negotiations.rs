use crate::models::ContractNegotiationItem;
use patternfly_yew::prelude::*;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ListContractNegotiationsProps {
  pub contract_negotiation_items: Vec<ContractNegotiationItem>,
  pub offset: usize,
  pub limit: usize,
  pub switch: bool,
  pub onoffset: Callback<usize>,
  pub onlimit: Callback<usize>,
  pub onswitch: Callback<bool>,
  pub on_show_contract_negotiation: Callback<String>,
}

#[component]
pub fn ListContractNegotiations(props: &ListContractNegotiationsProps) -> Html {
  let header = html_nested! {
    <TableHeader<Columns>>
      <TableColumn<Columns> label="State" index={Columns::State} />
      <TableColumn<Columns> label="Contract Agreement ID" index={Columns::ContractAgreementId} />
      <TableColumn<Columns> label="Counter Party ID" index={Columns::CounterPartyId} />
      <TableColumn<Columns> label="Protocol" index={Columns::Protocol} />
      <TableColumn<Columns> label="Kind" index={Columns::Kind} />
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

  let switch_callback = use_callback(
    props.onswitch.clone(),
    |switch, onswitch| {
      onswitch.emit(switch);
    },
  );

  let rows = props
    .contract_negotiation_items
    .iter()
    .map(
      |contract_negotiation_item| ContractNegotiationItemRenderer {
        item: contract_negotiation_item.clone(),
        on_show_contract_negotiation: props.on_show_contract_negotiation.clone(),
      },
    )
    .collect();

  let (entries, _) = use_table_data(MemoizedTableModel::new(Rc::new(rows)));

  html!(
    <>
      <Toolbar>
        <ToolbarContent>
          <ToolbarItem r#type={ToolbarItemType::Pagination}>
            <Switch label="as Consumer" label_off="as Provider" onchange={&switch_callback}/>
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
  State,
  ContractAgreementId,
  CounterPartyId,
  Protocol,
  Kind,
  Actions,
}

#[derive(Clone, Debug)]
struct ContractNegotiationItemRenderer {
  item: ContractNegotiationItem,
  on_show_contract_negotiation: Callback<String>,
}

impl TableEntryRenderer<Columns> for ContractNegotiationItemRenderer {
  fn render_cell(&self, context: CellContext<'_, Columns>) -> Cell {
    match context.column {
      Columns::State => html! { <Label label={self.item.state.to_string()} color={Color::Blue} /> },
      Columns::ContractAgreementId => html! { self.item.contract_agreement_id.to_string() },
      Columns::CounterPartyId => html! { self.item.counter_party_id.to_string() },
      Columns::Protocol => html! { self.item.protocol.to_string() },
      Columns::Kind => html! { self.item.kind.to_string() },
      Columns::Actions => {
        let contract_negotiation_id = self.item.id.clone();
        html! {
          <Button
            variant={ButtonVariant::Primary}
            onclick={self.on_show_contract_negotiation.clone().reform(move |_| contract_negotiation_id.clone())}
            icon={Icon::Eye}
          >
            { "Show" }
          </Button>
        }
      },
    }
    .into()
  }
}
