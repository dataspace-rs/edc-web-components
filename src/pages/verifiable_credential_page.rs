use crate::components::ListVerifiableCredentials;
use crate::contexts::use_edc_identity_hub_context;
use crate::models::VerifiableCredential;
use patternfly_yew::prelude::*;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct VerifiableCredentialPageProps {
  pub on_new_verifiable_credential: Callback<()>,
  pub onshow: Callback<String>,
}

#[component]
pub fn VerifiableCredentialPage(props: &VerifiableCredentialPageProps) -> Html {
  let refresh = use_state(|| 0usize);

  let edc_identity_hub_context = use_edc_identity_hub_context();

  let ondelete = use_callback(
    (refresh.clone(), edc_identity_hub_context),
    |verifiable_credential_id: String, (refresh, edc_identity_hub_context)| {
      let refresh = refresh.clone();
      let verifiable_credential_id = verifiable_credential_id.clone();
      let edc_identity_hub_context = edc_identity_hub_context.clone();

      spawn_local(async move {
        let identity_hub_client = edc_identity_hub_context.get_client();

        identity_hub_client
          .delete_credential(
            edc_identity_hub_context.participant_id(),
            &verifiable_credential_id,
          )
          .await
          .unwrap_or_default();

        log::warn!(
          "Deleted Verifiable Credential {} - {}",
          verifiable_credential_id,
          *refresh + 1
        );
        refresh.set(*refresh + 1);
      });
    },
  );

  let onclick = use_callback(
    props.on_new_verifiable_credential.clone(),
    |_, on_new_verifiable_credential| {
      on_new_verifiable_credential.emit(());
    },
  );

  html!(
    <Stack gutter=true>
      <StackItem>
        <Split gutter=true>
          <SplitItem fill=true>
            <Title level={Level::H3} size={Size::XXLarge}>{ "Verifiable Credentials" }</Title>
          </SplitItem>
          <SplitItem>
            <Button icon={Icon::Plus} {onclick} variant={ButtonVariant::Primary}>
              { "Request a Verifiable Credential" }
            </Button>
          </SplitItem>
        </Split>
      </StackItem>
      <StackItem>
        <Suspense>
          <VerifiableCredentialPageInner
            {ondelete}
            onshow={props.onshow.clone()}
            force_refresh={*refresh}
          />
        </Suspense>
      </StackItem>
    </Stack>
  )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct VerifiableCredentialPageInnerProps {
  pub ondelete: Callback<String>,
  pub onshow: Callback<String>,
  pub force_refresh: usize,
}

#[component]
pub fn VerifiableCredentialPageInner(props: &VerifiableCredentialPageInnerProps) -> HtmlResult {
  let edc_identity_hub_context = use_edc_identity_hub_context();

  let verifiable_credential_items = use_future_with(
    (edc_identity_hub_context, props.force_refresh),
    |parameters| async move {
      let (edc_identity_hub_context, _) = (*parameters).clone();
      let identity_hub_client = edc_identity_hub_context.get_client();

      identity_hub_client
        .get_credentials(edc_identity_hub_context.participant_id())
        .await
        .unwrap_or_default()
        .into_iter()
        .map(VerifiableCredential::from)
        .collect::<Vec<_>>()
    },
  )?;

  let verifiable_credential_items = (*verifiable_credential_items).clone();

  Ok(html!(
    <ListVerifiableCredentials
      verifiable_credential_items={verifiable_credential_items}
      ondelete={props.ondelete.clone()}
      onshow={props.onshow.clone()}
    />
  ))
}
