use crate::models::Participant;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct DidResolverState {
  participants: Vec<Participant>,
}

impl DidResolverState {
  pub fn participant(&self, participant_did: &str) -> Option<&Participant> {
    self
      .participants
      .iter()
      .find(|participant| participant.did == participant_did)
  }
}

#[derive(Properties, PartialEq)]
pub struct DidResolverContextProviderProps {
  #[prop_or_default]
  pub children: Html,
  pub participants: Vec<Participant>,
}

#[component]
pub fn DidResolverContextProvider(props: &DidResolverContextProviderProps) -> Html {
  let did_resolver_context = use_state(move || DidResolverState {
    participants: props.participants.clone(),
  });

  use_effect_with(
    (props.participants.clone(), did_resolver_context.setter()),
    |(participants, did_resolver_context_setter)| {
      did_resolver_context_setter.set(DidResolverState {
        participants: participants.clone(),
      });
    },
  );

  html! {
    <ContextProvider<DidResolverState> context={(*did_resolver_context).clone()}>
      { props.children.clone() }
    </ContextProvider<DidResolverState>>
  }
}

#[hook]
pub fn use_did_resolver_context() -> Option<DidResolverState> {
  use_context::<DidResolverState>()
}
