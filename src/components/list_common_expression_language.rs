use crate::models::CommonExpressionLanguageItem;
use patternfly_yew::prelude::*;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ListCommonExpressionLanguageProps {
  pub common_expression_language_items: Vec<CommonExpressionLanguageItem>,
  pub offset: usize,
  pub limit: usize,
  pub onoffset: Callback<usize>,
  pub onlimit: Callback<usize>,
  pub ondelete: Callback<String>,
}

#[component]
pub fn ListCommonExpressionLanguage(props: &ListCommonExpressionLanguageProps) -> Html {
  let header = html_nested! {
    <TableHeader<Columns>>
      <TableColumn<Columns> label="ID" index={Columns::Id} />
      <TableColumn<Columns> label="Left Operand" index={Columns::LeftOperand} />
      <TableColumn<Columns> label="Description" index={Columns::Description} />
      <TableColumn<Columns> label="Scopes" index={Columns::Scopes} />
      <TableColumn<Columns> label="Expression" index={Columns::Expression} />
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
    .common_expression_language_items
    .iter()
    .map(
      |common_expression_language_item| ListCommonExpressionLanguageRenderer {
        common_expression_language: common_expression_language_item.clone(),
        ondelete: props.ondelete.clone(),
      },
    )
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
      <Table<Columns, UseTableData<Columns, MemoizedTableModel<ListCommonExpressionLanguageRenderer>>> mode={TableMode::Compact} {header} {entries} />
    </>
  )
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Columns {
  Id,
  LeftOperand,
  Description,
  Scopes,
  Expression,
  Actions,
}

#[derive(Clone, Debug)]
struct ListCommonExpressionLanguageRenderer {
  common_expression_language: CommonExpressionLanguageItem,
  ondelete: Callback<String>,
}

impl TableEntryRenderer<Columns> for ListCommonExpressionLanguageRenderer {
  fn render_cell(&self, context: CellContext<'_, Columns>) -> Cell {
    match context.column {
      Columns::Id => html! { self.common_expression_language.id.to_owned() },
      Columns::LeftOperand => html!(self.common_expression_language.left_operand.to_owned()),
      Columns::Description => html!(
        self
          .common_expression_language
          .description
          .clone()
          .unwrap_or_default()
      ),
      Columns::Scopes => html!(self.common_expression_language.scopes.to_owned().join(", ")),
      Columns::Expression => html!(
        <ShowExpression expression={self.common_expression_language.expression.to_owned()} />
      ),
      Columns::Actions => {
        let common_expression_language_id = self.common_expression_language.id.to_string();

        html!(
          <DeleteCommonExpressionLanguage
            {common_expression_language_id}
            ondelete={self.ondelete.clone()}
          />
        )
      }
    }
    .into()
  }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ShowExpressionProps {
  pub expression: String,
}

#[function_component]
pub fn ShowExpression(props: &ShowExpressionProps) -> Html {
  let backdropper = use_backdrop();

  let onclick = use_callback(
    (backdropper.clone(), props.expression.clone()),
    |_, (backdropper, expression)| {
      let expression = expression.clone();
      if let Some(backdropper) = backdropper {
        backdropper.open(html!(
          <Bullseye>
            <Modal variant={ModalVariant::Large} title="Common Expression Language">
              <CodeBlock>
                <CodeBlockCode>{ expression }</CodeBlockCode>
              </CodeBlock>
            </Modal>
          </Bullseye>
        ))
      }
    },
  );

  html!(<Button variant={ButtonVariant::Primary} icon={Icon::Eye} {onclick}>{ "Show" }</Button>)
}

#[derive(Clone, PartialEq, Properties)]
pub struct DeleteCommonExpressionLanguageProps {
  pub common_expression_language_id: String,
  pub ondelete: Callback<String>,
}

#[function_component]
pub fn DeleteCommonExpressionLanguage(props: &DeleteCommonExpressionLanguageProps) -> Html {
  let onclick = use_callback(
    (
      props.common_expression_language_id.clone(),
      props.ondelete.clone(),
    ),
    |_, (common_expression_language_id, ondelete)| {
      ondelete.emit(common_expression_language_id.to_string());
    },
  );

  html!(<Button variant={ButtonVariant::Danger} icon={Icon::Trash} {onclick}>{ "Delete" }</Button>)
}
