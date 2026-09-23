use crate::components::ShowPolicy;
use crate::contexts::{RedirectionAction, use_redirection_context};
use edc_connector_client::types::policy::Policy;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ShowPolicyReferenceProps {
  pub policy: Policy,
}

#[component]
pub fn ShowPolicyReference(props: &ShowPolicyReferenceProps) -> Html {
  let backdropper = use_backdrop();
  let redirection_context = use_redirection_context();

  let onclick = use_callback(
    (
      backdropper.clone(),
      redirection_context.clone(),
      props.policy.clone(),
    ),
    |_, (backdropper, redirection_context, policy)| {
      if let Some(backdropper) = backdropper {
        let policy_id = policy.id().cloned().unwrap_or_default();
        let policy = policy.clone();

        let button = if let Some(redirection_context) = redirection_context {
          let backdropper = backdropper.clone();
          let redirection_context = redirection_context.clone();

          html!(
            <Button
              variant={ButtonVariant::Primary}
              icon={Icon::Eye}
              onclick={redirection_context.redirect_to().reform(move |_| {
            backdropper.close();
            RedirectionAction::Policy(policy_id.clone())
          })}
            >
              { "Show" }
            </Button>
          )
        } else {
          html!()
        };

        backdropper.open(Backdrop::new(html!(
          <Bullseye>
            <Modal title="Policy" variant={ModalVariant::Medium}>
              <Bullseye>
                <Stack gutter=true>
                  <StackItem>
                    <ShowPolicy {policy} />
                  </StackItem>
                  <StackItem>{ button }</StackItem>
                </Stack>
              </Bullseye>
            </Modal>
          </Bullseye>
        )));
      }
    },
  );

  html!(
    <Button variant={ButtonVariant::InlineLink} {onclick}>
      <Flex>
        <FlexItem modifiers={[FlexModifier::Align(Alignment::Center).all()]}>
          <yew_icons::Icon data={yew_icons::IconData::LUCIDE_SHIELD} />
        </FlexItem>
        <FlexItem modifiers={[FlexModifier::Align(Alignment::Center).all()]}>{ "Policy" }</FlexItem>
      </Flex>
    </Button>
  )
}
