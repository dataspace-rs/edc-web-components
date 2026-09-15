use crate::components::DidLabel;
use crate::models::VerifiableCredential;
use patternfly_yew::prelude::*;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ListVerifiableCredentialsProps {
  pub verifiable_credential_items: Vec<VerifiableCredential>,
  pub ondelete: Callback<String>,
  pub onshow: Callback<String>,
}

#[component]
pub fn ListVerifiableCredentials(props: &ListVerifiableCredentialsProps) -> Html {
  let header = html_nested! {
    <TableHeader<Columns>>
      <TableColumn<Columns> label="Issuer" index={Columns::IssuerId} />
      <TableColumn<Columns> label="Holder" index={Columns::HolderId} />
      <TableColumn<Columns> label="Insurance Date" index={Columns::InsuranceDate} />
      <TableColumn<Columns> label="Expiration Date" index={Columns::ExpirationDate} />
      <TableColumn<Columns> label="" index={Columns::Actions} />
    </TableHeader<Columns>>
  };

  let (entries, _) = use_table_data(MemoizedTableModel::new(Rc::new(
    props
      .verifiable_credential_items
      .clone()
      .into_iter()
      .map(|verifiable_credential| VerifiableCredentialRenderer {
        verifiable_credential,
        onshow: props.onshow.clone(),
        ondelete: props.ondelete.clone(),
      })
      .collect::<Vec<_>>(),
  )));

  html!(
    <Table<Columns, UseTableData<Columns, MemoizedTableModel<VerifiableCredentialRenderer>>>
      mode={TableMode::Compact}
      {header}
      {entries}
    />
  )
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Columns {
  IssuerId,
  HolderId,
  InsuranceDate,
  ExpirationDate,
  Actions,
}

#[derive(Clone, Debug)]
struct VerifiableCredentialRenderer {
  verifiable_credential: VerifiableCredential,
  onshow: Callback<String>,
  ondelete: Callback<String>,
}

impl TableEntryRenderer<Columns> for VerifiableCredentialRenderer {
  fn render_cell(&self, context: CellContext<'_, Columns>) -> Cell {
    match context.column {
      Columns::IssuerId => html! {
        <DidLabel did={self.verifiable_credential.issuer_id.to_string()} />
      },
      Columns::HolderId => html! {
        <DidLabel did={self.verifiable_credential.holder_id.to_string()} />
      },
      Columns::InsuranceDate => html! {
        self.verifiable_credential.issuance_date.format("%Y-%m-%d %H:%M:%S")
      },
      Columns::ExpirationDate => html! {
        self.verifiable_credential.expiration_date.format("%Y-%m-%d %H:%M:%S")
      },
      Columns::Actions => html! {
        <Split gutter=true>
          <SplitItem>
            <ShowVerifiableCredential
              verifiable_credential_id={self.verifiable_credential.id.clone()}
              onshow={self.onshow.clone()}
            />
          </SplitItem>
          <SplitItem>
            <DeleteVerifiableCredential
              verifiable_credential_id={self.verifiable_credential.id.clone()}
              ondelete={self.ondelete.clone()}
            />
          </SplitItem>
        </Split>
      },
    }
    .into()
  }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ShowVerifiableCredentialProps {
  pub verifiable_credential_id: String,
  pub onshow: Callback<String>,
}

#[function_component]
pub fn ShowVerifiableCredential(props: &ShowVerifiableCredentialProps) -> Html {
  let onclick = use_callback(
    (props.onshow.clone(), props.verifiable_credential_id.clone()),
    move |_, (onshow, verifiable_credential_id)| {
      onshow.emit(verifiable_credential_id.to_string());
    },
  );

  html!(<Button variant={ButtonVariant::Primary} icon={Icon::Eye} {onclick}>{ "Show" }</Button>)
}

#[derive(Clone, PartialEq, Properties)]
pub struct DeleteVerifiableCredentialProps {
  pub verifiable_credential_id: String,
  pub ondelete: Callback<String>,
}

#[function_component]
pub fn DeleteVerifiableCredential(props: &DeleteVerifiableCredentialProps) -> Html {
  let onclick = use_callback(
    (
      props.ondelete.clone(),
      props.verifiable_credential_id.clone(),
    ),
    move |_, (ondelete, verifiable_credential_id)| {
      ondelete.emit(verifiable_credential_id.to_string());
    },
  );

  html!(<Button variant={ButtonVariant::Danger} icon={Icon::Trash} {onclick}>{ "Delete" }</Button>)
}
