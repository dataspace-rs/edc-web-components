use crate::models::Participant;
use std::{ops::Deref, rc::Rc};
use yew::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum DidResolverAction {}

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

impl Reducible for DidResolverState {
  type Action = DidResolverAction;

  fn reduce(self: Rc<Self>, _action: Self::Action) -> Rc<Self> {
    let new_self = self.deref().clone();

    new_self.into()
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
  let did_resolver_context = use_reducer(move || DidResolverState {
    participants: props.participants.clone(),
  });

  html! {
    <ContextProvider<DidResolverContext> context={did_resolver_context}>
      { props.children.clone() }
    </ContextProvider<DidResolverContext>>
  }
}

pub type DidResolverContext = UseReducerHandle<DidResolverState>;

#[hook]
pub fn use_did_resolver_context() -> Option<DidResolverContext> {
  use_context::<DidResolverContext>()
}
