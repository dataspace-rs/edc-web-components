use edc_federated_catalog_client::models::FederatedCatalogParticipant;
use patternfly_yew::prelude::*;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ListFederatedCatalogParticipantsProps {
  pub federated_catalog_participants: Vec<FederatedCatalogParticipant>,
  pub ondelete: Callback<FederatedCatalogParticipant>,
  #[prop_or_default]
  pub on_show_offer: Option<Callback<FederatedCatalogParticipant>>,
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
      .map(
        |federated_catalog_participant| FederatedCatalogParticipantRenderer {
          federated_catalog_participant,
          ondelete: props.ondelete.clone(),
          on_show_offer: props.on_show_offer.clone(),
        },
      )
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
struct FederatedCatalogParticipantRenderer {
  federated_catalog_participant: FederatedCatalogParticipant,
  ondelete: Callback<FederatedCatalogParticipant>,
  on_show_offer: Option<Callback<FederatedCatalogParticipant>>,
}

impl TableEntryRenderer<Columns> for FederatedCatalogParticipantRenderer {
  fn render_cell(&self, context: CellContext<'_, Columns>) -> Cell {
    match context.column {
      Columns::Name => html! { self.federated_catalog_participant.name.to_string() },
      Columns::CounterPartyDid => html! { self.federated_catalog_participant.id.to_string() },
      Columns::CounterPartyAddress => html! {
        self.federated_catalog_participant.target_url.to_string()
      },
      Columns::Actions => {
        let federated_catalog_participant = self.federated_catalog_participant.clone();

        let show_offer_button = if let Some(on_show_offer) = self.on_show_offer.clone() {
          let federated_catalog_participant = federated_catalog_participant.clone();

          html!(
            <Button
              variant={ButtonVariant::Primary}
              icon={Icon::Eye}
              onclick={on_show_offer.reform(move |_| federated_catalog_participant.clone())}
            >
              { "Show Offers" }
            </Button>
          )
        } else {
          html!()
        };

        html!(
          <Flex>
            <FlexItem>{ show_offer_button }</FlexItem>
            <FlexItem>
              <Button
                variant={ButtonVariant::Danger}
                icon={Icon::Trash}
                onclick={self.ondelete.reform(move |_| federated_catalog_participant.clone())}
              >
                { "Trash" }
              </Button>
            </FlexItem>
          </Flex>
        )
      }
    }
    .into()
  }
}
