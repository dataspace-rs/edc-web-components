use crate::components::show_policy_reference::ShowPolicyReference;
use crate::components::{AssetReference, DidLabel};
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
  pub onshow: Callback<String>,
}

#[component]
pub fn ListContractAgreements(props: &ListContractAgreementsProps) -> Html {
  let header = html_nested! {
    <TableHeader<Columns>>
      <TableColumn<Columns> label="Contract Signing Date" index={Columns::ContractSigningDate} />
      <TableColumn<Columns> label="Consumer" index={Columns::Consumer} />
      <TableColumn<Columns> label="Provider" index={Columns::Provider} />
      <TableColumn<Columns> label="Asset" index={Columns::Asset} />
      <TableColumn<Columns> label="Policy" index={Columns::Policy} />
      <TableColumn<Columns> label="" index={Columns::Action} />
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
    .map(|contract_agreement_item| ContractAgreementItemRenderer {
      item: contract_agreement_item.clone(),
      onshow: props.onshow.clone(),
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
  ContractSigningDate,
  Consumer,
  Provider,
  Asset,
  Policy,
  Action,
}

#[derive(Clone, Debug)]
struct ContractAgreementItemRenderer {
  item: ContractAgreementItem,
  onshow: Callback<String>,
}

impl ContractAgreementItemRenderer {}

impl TableEntryRenderer<Columns> for ContractAgreementItemRenderer {
  fn render_cell(&self, context: CellContext<'_, Columns>) -> Cell {
    match context.column {
      Columns::ContractSigningDate => html!(self.item.signing_date.to_string()),
      Columns::Consumer => html! { <DidLabel did={self.item.consumer_id.to_string()} /> },
      Columns::Provider => html! { <DidLabel did={self.item.provider_id.to_string()} /> },
      Columns::Asset => html!(<AssetReference asset_id={self.item.asset_id.to_string()} />),
      Columns::Policy => html!(<ShowPolicyReference policy={self.item.policy.clone()} />),
      Columns::Action => {
        let id = self.item.id.clone();
        html!(
          <Button
            variant={ButtonVariant::Primary}
            onclick={self.onshow.reform(move |_| id.clone())}
            icon={Icon::Eye}
          >
            { "Show" }
          </Button>
        )
      }
    }
    .into()
  }
}
