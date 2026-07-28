use crate::components::{CreateFederatedCatalogParticipant, ListFederatedCatalogParticipants};
use edc_federated_catalog_client::models::FederatedCatalogParticipant;
use edc_federated_catalog_client::{FederatedCatalogClient, FederatedCatalogClientVersion};
use patternfly_yew::prelude::*;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::suspense::use_future_with;
use yew_oauth2::hook::use_latest_access_token;

#[component]
pub fn FederatedCatalogManagementPage() -> Html {
  let refresh = use_state(|| 0usize);
  let backdropper = use_backdrop();

  let on_create = use_callback(
    (backdropper.clone(), refresh.clone()),
    |_, (backdropper, refresh)| {
      if let Some(backdropper) = backdropper {
        backdropper.close();
      }

      refresh.set(**refresh + 1);
    },
  );

  let onclick = use_callback((backdropper, on_create), |_, (backdropper, on_create)| {
    if let Some(backdropper) = backdropper {
      backdropper.open(html!(
        <Bullseye>
          <Modal variant={ModalVariant::Medium} title="Create a Participant">
            <CreateFederatedCatalogParticipant {on_create} />
          </Modal>
        </Bullseye>
      ))
    }
  });

  let refresh_asked = use_callback(refresh.clone(), |_, refresh| {
    refresh.set(**refresh + 1);
  });

  html!(
    <Stack gutter=true>
      <StackItem>
        <Split gutter=true>
          <SplitItem fill=true>
            <Title level={Level::H3} size={Size::XXLarge}>
              { "Federated Catalog Participants" }
            </Title>
          </SplitItem>
          <SplitItem>
            <Button icon={Icon::Plus} {onclick} variant={ButtonVariant::Primary}>{ "Add" }</Button>
          </SplitItem>
        </Split>
      </StackItem>
      <StackItem>
        <Suspense>
          <ListFederatedCatalogParticipantsInner force_refresh={*refresh} {refresh_asked} />
        </Suspense>
      </StackItem>
    </Stack>
  )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ListFederatedCatalogParticipantsInnerProps {
  force_refresh: usize,
  refresh_asked: Callback<()>,
}

#[component]
pub fn ListFederatedCatalogParticipantsInner(
  props: &ListFederatedCatalogParticipantsInnerProps,
) -> HtmlResult {
  let latest_access_token_context = use_latest_access_token().unwrap();

  let federated_catalog_participants = use_future_with(
    (latest_access_token_context.clone(), props.force_refresh),
    |parameters| async move {
      let (latest_access_token_context, _) = (*parameters).clone();

      let server_url = web_sys::window().unwrap().location().origin().unwrap();
      let federated_catalog_client = FederatedCatalogClient::new(
        reqwest::Client::new(),
        format!("{server_url}/federated-catalog-management"),
        latest_access_token_context.access_token(),
        FederatedCatalogClientVersion::V4,
      );

      federated_catalog_client
        .list_participants()
        .await
        .unwrap_or_default()
    },
  )?;

  let federated_catalog_participants = (*federated_catalog_participants).clone();

  let ondelete = use_callback(
    (
      latest_access_token_context.clone(),
      props.refresh_asked.clone(),
    ),
    |federated_catalog_participant: FederatedCatalogParticipant,
     (latest_access_token_context, refresh_asked)| {
      let server_url = web_sys::window().unwrap().location().origin().unwrap();
      let federated_catalog_client = FederatedCatalogClient::new(
        reqwest::Client::new(),
        format!("{server_url}/federated-catalog-management"),
        latest_access_token_context.access_token(),
        FederatedCatalogClientVersion::V4,
      );
      let refresh_asked = refresh_asked.clone();

      spawn_local(async move {
        if federated_catalog_client
          .delete_participant(&federated_catalog_participant.id)
          .await
          .is_ok()
        {
          refresh_asked.emit(());
        }
      });
    },
  );

  Ok(html!(<ListFederatedCatalogParticipants {federated_catalog_participants} {ondelete} />))
}
