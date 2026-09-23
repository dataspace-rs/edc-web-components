use std::{ops::Deref, rc::Rc};
use yew::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum RedirectionAction {
  Assets,
  Asset(String),
  Policies,
  Policy(String),
  ContractDefinitions,
  ContractNegotiations,
  ContractNegotiation(String),
  ContractAgreements,
  ContractAgreement(String),
  TransferProcesses,
  TransferProcess(String),
  VerifiableCredentials,
  VerifiableCredential(String),
}

#[derive(Clone, PartialEq)]
pub struct RedirectionState {
  redirect_to: Callback<RedirectionAction>,
}

impl RedirectionState {
  pub fn redirect_to(&self) -> Callback<RedirectionAction> {
    self.redirect_to.clone()
  }
}

impl Reducible for RedirectionState {
  type Action = RedirectionAction;

  fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
    let new_self = self.deref().clone();
    new_self.redirect_to.emit(action);
    new_self.into()
  }
}

#[derive(Properties, PartialEq)]
pub struct RedirectionContextProviderProps {
  #[prop_or_default]
  pub children: Html,
  pub redirect_to: Callback<RedirectionAction>,
}

#[component]
pub fn RedirectionContextProvider(props: &RedirectionContextProviderProps) -> Html {
  let redirection_context = use_reducer(move || RedirectionState {
    redirect_to: props.redirect_to.clone(),
  });

  html! {
    <ContextProvider<RedirectionContext> context={redirection_context}>
      { props.children.clone() }
    </ContextProvider<RedirectionContext>>
  }
}

pub type RedirectionContext = UseReducerHandle<RedirectionState>;

#[hook]
pub fn use_redirection_context() -> Option<RedirectionContext> {
  use_context::<RedirectionContext>()
}
