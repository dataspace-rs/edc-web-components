use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct MyDidProviderState {
  my_did: String,
}

impl MyDidProviderState {
  pub fn my_did(&self) -> &str {
    &self.my_did
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
  let my_did_provider_context = use_state(move || MyDidProviderState {
    my_did: props.my_did.clone(),
  });

  use_effect_with(
    (props.my_did.clone(), my_did_provider_context.setter()),
    |(my_did, my_did_provider_context_setter)| {
      my_did_provider_context_setter.set(MyDidProviderState {
        my_did: my_did.clone(),
      });
    },
  );

  html! {
    <ContextProvider<MyDidProviderState> context={(*my_did_provider_context).clone()}>
      { props.children.clone() }
    </ContextProvider<MyDidProviderState>>
  }
}

#[hook]
pub fn use_my_did_provider_context() -> Option<MyDidProviderState> {
  use_context::<MyDidProviderState>()
}
