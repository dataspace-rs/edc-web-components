use edc_federated_catalog_client::models::FederatedCatalogParticipant;
use patternfly_yew::prelude::*;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ListFederatedCatalogParticipantsProps {
  pub federated_catalog_participants: Vec<FederatedCatalogParticipant>,
  pub ondelete: Callback<FederatedCatalogParticipant>,
}

#[component]
pub fn ListFederatedCatalogParticipants(props: &ListFederatedCatalogParticipantsProps) -> Html {
  let header = html_nested! {
    <TableHeader<Columns>>
      <TableColumn<Columns> label="Counter Party Name" index={Columns::Name} />
      <TableColumn<Columns> label="Counter Party DID" index={Columns::CounterPartyDid} />
      <TableColumn<Columns> label="Counter Party Address" index={Columns::CounterPartyAddress} />
      <TableColumn<Columns> label="" index={Columns::Actions} />
    </TableHeader<Columns>>
  };

  let (entries, _) = use_table_data(MemoizedTableModel::new(Rc::new(
    props
      .federated_catalog_participants
      .clone()
      .into_iter()
      .map(|federated_catalog_participant| {
        FederatedCatalogParticipantRenderer((federated_catalog_participant, props.ondelete.clone()))
      })
      .collect(),
  )));

  html!(
    <Table<Columns, UseTableData<Columns, MemoizedTableModel<FederatedCatalogParticipantRenderer>>>
      mode={TableMode::Compact}
      {header}
      {entries}
    />
  )
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Columns {
  Name,
  CounterPartyDid,
  CounterPartyAddress,
  Actions,
}

#[derive(Clone, Debug)]
struct FederatedCatalogParticipantRenderer(
  (
    FederatedCatalogParticipant,
    Callback<FederatedCatalogParticipant>,
  ),
);

impl TableEntryRenderer<Columns> for FederatedCatalogParticipantRenderer {
  fn render_cell(&self, context: CellContext<'_, Columns>) -> Cell {
    match context.column {
      Columns::Name => html! { self.0.0.name.to_string() },
      Columns::CounterPartyDid => html! { self.0.0.id.to_string() },
      Columns::CounterPartyAddress => html! { self.0.0.target_url.to_string() },
      Columns::Actions => {
        let participant = self.0.0.clone();

        html!(
          <DeleteFederatedCatalogParticipant
            ondelete={self.0.1.reform(move |_| participant.clone())}
          />
        )
      }
    }
    .into()
  }
}

#[derive(Clone, PartialEq, Properties)]
pub struct DeleteFederatedCatalogParticipantProps {
  pub ondelete: Callback<()>,
}

#[function_component]
pub fn DeleteFederatedCatalogParticipant(props: &DeleteFederatedCatalogParticipantProps) -> Html {
  let onclick = use_callback(props.ondelete.clone(), |_, ondelete| {
    ondelete.emit(());
  });

  html!(<Button variant={ButtonVariant::Danger} icon={Icon::Trash} {onclick}>{ "Delete" }</Button>)
}
