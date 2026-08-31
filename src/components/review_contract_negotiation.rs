use crate::components::ShowPolicy;
use crate::contexts::use_edc_connector_context;
use patternfly_yew::prelude::*;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ReviewContractNegotiationProps {
  pub contract_negotiation_id: String,
  pub on_reviewed: Callback<()>,
}

#[component]
pub fn ReviewContractNegotiation(props: &ReviewContractNegotiationProps) -> HtmlResult {
  let edc_connector_client = use_edc_connector_context();

  let policy = use_future_with(
    (
      props.contract_negotiation_id.clone(),
      edc_connector_client.clone(),
    ),
    |properties| async move {
      let (contract_negotiation_id, edc_connector_client) = (*properties).clone();

      if let Some(client) = edc_connector_client.get_client() {
        match client
          .contract_negotiations(edc_connector_client::EdcConnectorApiVersion::V4)
          .review(&contract_negotiation_id)
          .await
        {
          Ok(policy) => Some(policy),
          Err(error) => {
            log::error!("Error reviewing contract negotiation: {:?}", error);
            None
          }
        }
      } else {
        None
      }
    },
  )?;

  let policy = (*policy).clone();

  let on_approve = use_callback(
    (
      props.contract_negotiation_id.clone(),
      edc_connector_client.clone(),
      props.on_reviewed.clone(),
    ),
    |_, (contract_negotiation_id, edc_connector_client, on_reviewed)| {
      let contract_negotiation_id = contract_negotiation_id.clone();
      let edc_connector_client = edc_connector_client.clone();
      let on_reviewed = on_reviewed.clone();

      spawn_local(async move {
        if let Some(edc_connector_client) = edc_connector_client.get_client() {
          if let Err(error) = edc_connector_client
            .contract_negotiations(edc_connector_client::EdcConnectorApiVersion::V4)
            .approve(&contract_negotiation_id)
            .await
          {
            log::error!("Error approving contract negotiation: {:?}", error);
          } else {
            on_reviewed.emit(());
          }
        }
      });
    },
  );

  let rejection_message = use_state(String::new);

  let on_change_rejection_message = use_callback(
    rejection_message.setter(),
    |rejection_message, rejection_message_setter| {
      rejection_message_setter.set(rejection_message);
    },
  );

  let backdropper = use_backdrop();

  let on_reject = use_callback(
    (
      props.contract_negotiation_id.clone(),
      edc_connector_client.clone(),
      rejection_message.clone(),
      backdropper.clone(),
      props.on_reviewed.clone(),
    ),
    |event: SubmitEvent,
     (
      contract_negotiation_id,
      edc_connector_client,
      rejection_message,
      backdropper,
      on_reviewed,
    )| {
      event.prevent_default();
      let contract_negotiation_id = contract_negotiation_id.clone();
      let edc_connector_client = edc_connector_client.clone();
      let rejection_message = (**rejection_message).clone();
      let backdropper = backdropper.clone();
      let on_reviewed = on_reviewed.clone();

      spawn_local(async move {
        if let Some(edc_connector_client) = edc_connector_client.get_client() {
          if let Err(error) = edc_connector_client
            .contract_negotiations(edc_connector_client::EdcConnectorApiVersion::V4)
            .terminate(&contract_negotiation_id, &rejection_message)
            .await
          {
            log::error!("Error approving contract negotiation: {:?}", error);
          } else {
            if let Some(backdropper) = backdropper {
              backdropper.close();
              on_reviewed.emit(());
            }
          }
        }
      });
    },
  );

  let on_reject_modal = use_callback(
    (backdropper, rejection_message, on_reject.clone()),
    move |_, (backdropper, rejection_message, on_reject)| {
      if let Some(backdropper) = backdropper {
        let rejection_message = (**rejection_message).clone();

        backdropper.open(Backdrop::new(html!(
          <Bullseye>
            <Modal title="Reject Contract Negotiation" variant={ModalVariant::Medium}>
              <Form onsubmit={on_reject}>
                <FormGroup label="Reason" required=true>
                  <TextInput
                    placeholder="Write the rejection reason"
                    required=true
                    value={rejection_message}
                    onchange={on_change_rejection_message.clone()}
                  />
                </FormGroup>
                <ActionGroup>
                  <Button
                    variant={ButtonVariant::Primary}
                    label="Submit"
                    r#type={ButtonType::Submit}
                  />
                </ActionGroup>
              </Form>
            </Modal>
          </Bullseye>
        )));
      }
    },
  );

  if let Some(policy) = policy {
    Ok(html! {
      <Stack>
        <StackItem>
          <ShowPolicy {policy} />
        </StackItem>
        <StackItem>
          <Flex>
            <FlexItem modifiers={[FlexModifier::Grow]} />
            <FlexItem>
              <Button variant={ButtonVariant::Primary} icon={Icon::Check} onclick={on_approve}>
                { "Approve" }
              </Button>
            </FlexItem>
            <FlexItem>
              <Button variant={ButtonVariant::Danger} icon={Icon::Times} onclick={on_reject_modal}>
                { "Reject" }
              </Button>
            </FlexItem>
          </Flex>
        </StackItem>
      </Stack>
    })
  } else {
    Ok(html! { <div>{ "Unable to find Policy related to this Contract Negotiation" }</div> })
  }
}
