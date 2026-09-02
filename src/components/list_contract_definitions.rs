use crate::components::{AssetReference, PolicyReference};
use crate::models::ContractDefinitionItem;
use patternfly_yew::prelude::*;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ListContractDefinitionsProps {
  pub contract_definition_items: Vec<ContractDefinitionItem>,
  pub offset: usize,
  pub limit: usize,
  pub onoffset: Callback<usize>,
  pub onlimit: Callback<usize>,
  pub ondelete: Callback<String>,
  #[prop_or_default]
  pub on_policy_click: Callback<String>,
  #[prop_or_default]
  pub on_asset_click: Callback<String>,
}

#[component]
pub fn ListContractDefinitions(props: &ListContractDefinitionsProps) -> Html {
  let header = html_nested! {
    <TableHeader<Columns>>
      <TableColumn<Columns> label="Name" index={Columns::Name} />
      <TableColumn<Columns> label="Access Policy" index={Columns::AccessPolicy} />
      <TableColumn<Columns> label="Contract Policy" index={Columns::ContractPolicy} />
      <TableColumn<Columns> label="Assets" index={Columns::Assets} />
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
    .contract_definition_items
    .iter()
    .map(|contract_definition_item| ContractDefinitionItemRenderer {
      contract_definition_item: contract_definition_item.clone(),
      ondelete: props.ondelete.clone(),
      on_policy_click: props.on_policy_click.clone(),
      on_asset_click: props.on_asset_click.clone(),
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
      <Table<Columns, UseTableData<Columns, MemoizedTableModel<ContractDefinitionItemRenderer>>>
        mode={TableMode::Compact}
        {header}
        {entries}
      />
    </>
  )
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Columns {
  Name,
  AccessPolicy,
  ContractPolicy,
  Assets,
  Actions,
}

#[derive(Clone, Debug)]
struct ContractDefinitionItemRenderer {
  contract_definition_item: ContractDefinitionItem,
  ondelete: Callback<String>,
  on_policy_click: Callback<String>,
  on_asset_click: Callback<String>,
}

impl ContractDefinitionItemRenderer {}

impl TableEntryRenderer<Columns> for ContractDefinitionItemRenderer {
  fn render_cell(&self, context: CellContext<'_, Columns>) -> Cell {
    match context.column {
      Columns::Name => html! { self.contract_definition_item.name.to_string() },
      Columns::AccessPolicy => {
        let access_policy_id = self.contract_definition_item.access_policy_id.to_string();
        html!(
          <PolicyReference
            policy_id={self.contract_definition_item.access_policy_id.to_string()}
            on_click={self.on_policy_click.reform(move |_| access_policy_id.clone())}
          />
        )
      },
      Columns::ContractPolicy => {
        let contract_policy_id = self.contract_definition_item.contract_policy_id.to_string();
        html!(
          <PolicyReference
            policy_id={self.contract_definition_item.contract_policy_id.to_string()}
            on_click={self.on_policy_click.reform(move |_| contract_policy_id.clone())}
          />
        )
      },
      Columns::Assets => {
        let assets = self.contract_definition_item.asset_ids.iter().map(|asset_id| {
          let asset_id = asset_id.clone();

          html_nested!(
            <FlexItem>
              <AssetReference
                asset_id={asset_id.clone()}
                on_click={self.on_asset_click.reform(move |_| asset_id.clone())}
              />
            </FlexItem>
          )
        });

        html!(<Flex>{ for assets }</Flex>)
      },
      Columns::Actions => {
        let contract_definition_id = self.contract_definition_item.id.to_string();

        html!(<DeleteContractDefinition {contract_definition_id} ondelete={self.ondelete.clone()} />)
      }
    }
    .into()
  }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub contract_definition_id: String,
  pub ondelete: Callback<String>,
}

#[function_component]
pub fn DeleteContractDefinition(props: &Props) -> Html {
  let onclick = use_callback(
    (props.ondelete.clone(), props.contract_definition_id.clone()),
    move |_, (ondelete, contract_definition_id)| {
      ondelete.emit(contract_definition_id.to_string());
    },
  );

  html!(<Button variant={ButtonVariant::Danger} icon={Icon::Trash} {onclick}>{ "Delete" }</Button>)
}
