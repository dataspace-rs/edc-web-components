use crate::models::PolicyDefinitionItem;
use patternfly_yew::prelude::*;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ListPoliciesProps {
  pub policy_definition_items: Vec<PolicyDefinitionItem>,
  pub offset: usize,
  pub limit: usize,
  pub onoffset: Callback<usize>,
  pub onlimit: Callback<usize>,
  pub ondelete: Callback<String>,
  pub onshow: Callback<String>,
}

#[component]
pub fn ListPolicies(props: &ListPoliciesProps) -> Html {
  let header = html_nested! {
    <TableHeader<Columns>>
      <TableColumn<Columns> label="ID" index={Columns::Id} />
      <TableColumn<Columns> label="Kind" index={Columns::Kind} />
      <TableColumn<Columns> label="Assignee" index={Columns::Assignee} />
      <TableColumn<Columns> label="Assigner" index={Columns::Assigner} />
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
    .policy_definition_items
    .iter()
    .map(|policy_definition_item| PolicyDefinitionItemRenderer {
      policy_definition_item: policy_definition_item.clone(),
      ondelete: props.ondelete.clone(),
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
      <Table<Columns, UseTableData<Columns, MemoizedTableModel<PolicyDefinitionItemRenderer>>>
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
  Kind,
  Assignee,
  Assigner,
  Actions,
}

#[derive(Clone, Debug)]
struct PolicyDefinitionItemRenderer {
  policy_definition_item: PolicyDefinitionItem,
  ondelete: Callback<String>,
  onshow: Callback<String>,
}

impl TableEntryRenderer<Columns> for PolicyDefinitionItemRenderer {
  fn render_cell(&self, context: CellContext<'_, Columns>) -> Cell {
    match context.column {
      Columns::Id => html! { self.policy_definition_item.id.to_string() },
      Columns::Kind => html! { self.policy_definition_item.kind.to_string() },
      Columns::Assignee => html!(
        self
          .policy_definition_item
          .assignee
          .clone()
          .unwrap_or_default()
      ),
      Columns::Assigner => html!(
        self
          .policy_definition_item
          .assigner
          .clone()
          .unwrap_or_default()
      ),
      Columns::Actions => {
        let policy_id = self.policy_definition_item.id.to_string();
        let ondelete = self.ondelete.clone();
        let onshow = self.onshow.clone();

        html!(
          <Split gutter=true>
            <SplitItem>
              <ShowPolicy policy_id={policy_id.clone()} {onshow} />
            </SplitItem>
            <SplitItem>
              <DeletePolicy {policy_id} {ondelete} />
            </SplitItem>
          </Split>
        )
      }
    }
    .into()
  }
}

#[derive(Clone, PartialEq, Properties)]
pub struct DeletePolicyProps {
  pub policy_id: String,
  pub ondelete: Callback<String>,
}

#[function_component]
pub fn DeletePolicy(props: &DeletePolicyProps) -> Html {
  let onclick = use_callback(
    (props.ondelete.clone(), props.policy_id.clone()),
    move |_, (ondelete, policy_id)| {
      ondelete.emit(policy_id.clone());
    },
  );

  html!(<Button variant={ButtonVariant::Danger} icon={Icon::Trash} {onclick}>{ "Delete" }</Button>)
}

#[derive(Clone, PartialEq, Properties)]
pub struct ShowPolicyProps {
  pub policy_id: String,
  pub onshow: Callback<String>,
}

#[function_component]
pub fn ShowPolicy(props: &ShowPolicyProps) -> Html {
  let onclick = use_callback(
    (props.onshow.clone(), props.policy_id.clone()),
    move |_, (onshow, policy_id)| {
      onshow.emit(policy_id.clone());
    },
  );

  html!(<Button variant={ButtonVariant::Primary} icon={Icon::Eye} {onclick}>{ "Show" }</Button>)
}
