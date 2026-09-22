use std::{ops::Deref, rc::Rc};
use yew::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum MyDidProviderAction {}

#[derive(Clone, PartialEq)]
pub struct MyDidProviderState {
  my_did: String,
}

impl MyDidProviderState {
  pub fn my_did(&self) -> &str {
    &self.my_did
  }
}

impl Reducible for MyDidProviderState {
  type Action = MyDidProviderAction;

  fn reduce(self: Rc<Self>, _action: Self::Action) -> Rc<Self> {
    let new_self = self.deref().clone();

    new_self.into()
  }
}

#[derive(Properties, PartialEq)]
pub struct MyDidProviderContextProviderProps {
  #[prop_or_default]
  pub children: Html,
  pub my_did: String,
}

#[component]
pub fn MyDidProviderContextProvider(props: &MyDidProviderContextProviderProps) -> Html {
  let my_did_provider_context = use_reducer(move || MyDidProviderState {
    my_did: props.my_did.clone(),
  });

  html! {
    <ContextProvider<MyDidProviderContext> context={my_did_provider_context}>
      { props.children.clone() }
    </ContextProvider<MyDidProviderContext>>
  }
}

pub type MyDidProviderContext = UseReducerHandle<MyDidProviderState>;

#[hook]
pub fn use_my_did_provider_context() -> Option<MyDidProviderContext> {
  use_context::<MyDidProviderContext>()
}
