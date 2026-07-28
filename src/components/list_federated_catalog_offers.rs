use edc_federated_catalog_client::models::FederatedCatalogOffer;
use patternfly_yew::prelude::*;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct SelectedFederatedCatalogOffer {
  pub originator: String,
  pub provider_id: String,
  pub dataset_id: String,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ListFederatedCatalogOffersProps {
  pub federated_catalog_offers: Vec<FederatedCatalogOffer>,
  pub onselectedoffer: Callback<SelectedFederatedCatalogOffer>,
}

#[component]
pub fn ListFederatedCatalogOffers(props: &ListFederatedCatalogOffersProps) -> Html {
  let header = html_nested! {
    <TableHeader<Columns>>
      <TableColumn<Columns> label="Provider ID" index={Columns::ProviderId} />
      <TableColumn<Columns> label="Dataset ID" index={Columns::DatasetId} />
      <TableColumn<Columns> label="Dataset Name" index={Columns::DatasetName} />
      <TableColumn<Columns> label="" index={Columns::Actions} />
    </TableHeader<Columns>>
  };

  let rows = props
    .federated_catalog_offers
    .iter()
    .flat_map(|federated_catalog_offer| {
      federated_catalog_offer
        .dataset
        .iter()
        .map(|dataset| FederatedCatalogOfferRenderer {
          originator: federated_catalog_offer.originator.clone(),
          provider_id: federated_catalog_offer.participant_id.id.clone(),
          dataset_id: dataset.id.clone(),
          dataset_name: dataset.name.clone(),
          onselectedoffer: props.onselectedoffer.clone(),
        })
        .collect::<Vec<_>>()
    })
    .collect();

  let (entries, _) = use_table_data(MemoizedTableModel::new(Rc::new(rows)));

  html!(
    <Table<Columns, UseTableData<Columns, MemoizedTableModel<FederatedCatalogOfferRenderer>>>
      mode={TableMode::Compact}
      {header}
      {entries}
    />
  )
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Columns {
  ProviderId,
  DatasetId,
  DatasetName,
  Actions,
}

#[derive(Clone, Debug)]
struct FederatedCatalogOfferRenderer {
  originator: String,
  provider_id: String,
  dataset_id: String,
  dataset_name: String,
  onselectedoffer: Callback<SelectedFederatedCatalogOffer>,
}

impl TableEntryRenderer<Columns> for FederatedCatalogOfferRenderer {
  fn render_cell(&self, context: CellContext<'_, Columns>) -> Cell {
    match context.column {
      Columns::ProviderId => html!(self.provider_id.to_owned()),
      Columns::DatasetId => html!(self.dataset_id.to_owned()),
      Columns::DatasetName => html!(self.dataset_name.to_owned()),
      Columns::Actions => {
        let originator = self.originator.clone();
        let provider_id = self.provider_id.clone();
        let dataset_id = self.dataset_id.clone();

        html!(
          <Negotiate
            onselected={self.onselectedoffer.reform(move |_| SelectedFederatedCatalogOffer {
            originator: originator.clone(),
            provider_id: provider_id.clone(),
            dataset_id: dataset_id.clone()
          } )}
          />
        )
      }
    }
    .into()
  }
}

#[derive(Clone, Debug, Properties, PartialEq)]
struct NegotiateProps {
  onselected: Callback<()>,
}

#[component]
fn Negotiate(props: &NegotiateProps) -> Html {
  html!(
    <Button variant={ButtonVariant::Primary} onclick={props.onselected.reform(|_| ())}>
      <yew_icons::Icon data={yew_icons::IconData::LUCIDE_HEART_HANDSHAKE} />
      { "Negotiate" }
    </Button>
  )
}
