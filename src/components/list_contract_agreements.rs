use crate::models::ContractAgreementItem;
use patternfly_yew::prelude::*;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ListContractAgreementsProps {
  pub contract_agreement_items: Vec<ContractAgreementItem>,
  pub offset: usize,
  pub limit: usize,
  pub onoffset: Callback<usize>,
  pub onlimit: Callback<usize>,
}

#[component]
pub fn ListContractAgreements(props: &ListContractAgreementsProps) -> Html {
  let header = html_nested! {
    <TableHeader<Columns>>
      <TableColumn<Columns> label="ID" index={Columns::Id} />
      <TableColumn<Columns> label="Contract Signing Date" index={Columns::ContractSigningDate} />
      <TableColumn<Columns> label="Consumer ID" index={Columns::ConsumerId} />
      <TableColumn<Columns> label="Provider ID" index={Columns::ProviderId} />
      <TableColumn<Columns> label="Asset ID" index={Columns::AssetId} />
      <TableColumn<Columns> label="Policy ID" index={Columns::PolicyID} />
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
    .contract_agreement_items
    .iter()
    .map(|contract_agreement_item| ContractAgreementItemRenderer(contract_agreement_item.clone()))
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
      <Table<Columns, UseTableData<Columns, MemoizedTableModel<ContractAgreementItemRenderer>>>
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
  ContractSigningDate,
  ConsumerId,
  ProviderId,
  AssetId,
  PolicyID,
}

#[derive(Clone, Debug)]
struct ContractAgreementItemRenderer(ContractAgreementItem);

impl ContractAgreementItemRenderer {}

impl TableEntryRenderer<Columns> for ContractAgreementItemRenderer {
  fn render_cell(&self, context: CellContext<'_, Columns>) -> Cell {
    match context.column {
      Columns::Id => html! { self.0.id.to_string() },
      Columns::ContractSigningDate => html!(self.0.signing_date.to_string()),
      Columns::ConsumerId => html!(self.0.consumer_id.to_string()),
      Columns::ProviderId => html! { self.0.provider_id.to_string() },
      Columns::AssetId => html!(self.0.asset_id.to_string()),
      Columns::PolicyID => html!(self.0.policy_id.to_string()),
    }
    .into()
  }
}
