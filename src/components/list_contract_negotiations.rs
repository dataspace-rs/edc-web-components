use crate::components::{AssetReference, ContractAgreementReference, DidLabel, MultiStateSelector};
use crate::models::{ConsumerProvider, ContractNegotiationItem};
use patternfly_yew::prelude::*;
use std::collections::HashSet;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ListContractNegotiationsProps {
  pub contract_negotiation_items: Vec<ContractNegotiationItem>,
  pub offset: usize,
  pub limit: usize,
  pub statuses: Vec<(String, bool)>,
  pub on_offset: Callback<usize>,
  pub on_limit: Callback<usize>,
  pub on_show_contract_negotiation: Callback<String>,
  pub on_statuses: Callback<Vec<(String, bool)>>,
  #[prop_or(true)]
  pub show_status_selector: bool,
  #[prop_or_default]
  pub consumer_provider: HashSet<ConsumerProvider>,
  pub on_consumer_provider: Callback<HashSet<ConsumerProvider>>,
}

#[component]
pub fn ListContractNegotiations(props: &ListContractNegotiationsProps) -> Html {
  let header = html_nested! {
    <TableHeader<Columns>>
      <TableColumn<Columns> label="State" index={Columns::State} />
      <TableColumn<Columns> label="Contract Agreement" index={Columns::ContractAgreementId} />
      <TableColumn<Columns> label="Counter Party" index={Columns::CounterParty} />
      <TableColumn<Columns> label="Asset" index={Columns::Asset} />
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
      props.on_offset.clone(),
    ),
    |page: Navigation, (offset, limit, total_entries, on_offset)| {
      let offset = match page {
        Navigation::First => 0,
        Navigation::Last => (total_entries.unwrap_or_default().saturating_sub(1) / limit) * limit,
        Navigation::Previous => *offset - limit,
        Navigation::Next => *offset + limit,
        Navigation::Page(n) => n * limit,
      };
      on_offset.emit(offset);
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

  let statuses_selector = if props.show_status_selector {
    html!(
      <MultiStateSelector
        selectable_items={props.statuses.clone()}
        on_selected={props.on_statuses.clone()}
        none_selected_label="All States"
        all_selected_label="All States"
      />
    )
  } else {
    html!()
  };

  let consumer_provider_select = {
    let onclick_consumer = {
      let consumer_provider = props.consumer_provider.clone();

      props.on_consumer_provider.reform(move |_| {
        let mut consumer_provider = consumer_provider.clone();
        if consumer_provider.contains(&ConsumerProvider::Consumer) {
          consumer_provider.remove(&ConsumerProvider::Consumer);
        } else {
          consumer_provider.insert(ConsumerProvider::Consumer);
        }

        consumer_provider
      })
    };

    let onclick_provider = {
      let consumer_provider = props.consumer_provider.clone();

      props.on_consumer_provider.reform(move |_| {
        let mut consumer_provider = consumer_provider.clone();
        if consumer_provider.contains(&ConsumerProvider::Provider) {
          consumer_provider.remove(&ConsumerProvider::Provider);
        } else {
          consumer_provider.insert(ConsumerProvider::Provider);
        }

        consumer_provider
      })
    };

    let text = {
      let state = if props.consumer_provider.is_empty() {
        HashSet::from([ConsumerProvider::Consumer, ConsumerProvider::Provider])
      } else {
        props.consumer_provider.clone()
      };

      state
        .into_iter()
        .map(|consumer_provider| consumer_provider.to_string())
        .collect::<Vec<String>>()
        .join(", ")
    };

    html!(
      <Dropdown {text}>
        <MenuAction
          onclick={onclick_consumer}
          selected={props.consumer_provider.is_empty() || props.consumer_provider.contains(&ConsumerProvider::Consumer)}
        >
          { "Consumer" }
        </MenuAction>
        <MenuAction
          onclick={onclick_provider}
          selected={props.consumer_provider.is_empty() || props.consumer_provider.contains(&ConsumerProvider::Provider)}
        >
          { "Provider" }
        </MenuAction>
      </Dropdown>
    )
  };

  html!(
    <>
      <Toolbar>
        <ToolbarContent>
          <ToolbarItem r#type={ToolbarItemType::BulkSelect}>
            { consumer_provider_select }
            { statuses_selector }
          </ToolbarItem>
          <ToolbarItem r#type={ToolbarItemType::Pagination}>
            <Pagination
              offset={props.offset}
              entries_per_page_choices={vec![5, 10, 25, 50, 100]}
              selected_choice={props.limit}
              onlimit={&props.on_limit}
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
  CounterParty,
  Asset,
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
      Columns::State => {
        let color = match self.item.state.as_str() {
          "Finalized" => Color::Green,
          "Terminated" => Color::Red,
          _ => Color::Blue,
        };

        html! { <Label label={self.item.state.to_string()} {color} /> }
      },
      Columns::ContractAgreementId => {
        if self.item.contract_agreement_id.is_empty() {
          html! { "-" }
        }
         else {
           html! {
             <ContractAgreementReference
               contract_agreement_id={self.item.contract_agreement_id.to_string()}
             />
           }
         }
      },
      Columns::CounterParty => html! { <DidLabel did={self.item.counter_party_id.to_string()} /> },
      Columns::Asset => html! { <AssetReference asset_id={self.item.asset_id.to_string()} /> },
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
