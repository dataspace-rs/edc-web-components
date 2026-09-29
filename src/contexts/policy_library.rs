use edc_connector_client::types::policy::Policy;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct PolicyLibraryItem {
  pub name: String,
  pub policy: Policy,
}

#[derive(Clone, PartialEq)]
pub struct PolicyLibraryState {
  policy_library_items: Vec<PolicyLibraryItem>,
}

impl PolicyLibraryState {
  pub fn policy_library(&self) -> &[PolicyLibraryItem] {
    &self.policy_library_items
  }
}

#[derive(Properties, PartialEq)]
pub struct PolicyLibraryContextProviderProps {
  #[prop_or_default]
  pub children: Html,
  pub policy_library_items: Vec<PolicyLibraryItem>,
}

#[component]
pub fn PolicyLibraryContextProvider(props: &PolicyLibraryContextProviderProps) -> Html {
  let context = use_state(move || PolicyLibraryState {
    policy_library_items: props.policy_library_items.clone(),
  });

  use_effect_with(
    (props.policy_library_items.clone(), context.setter()),
    |(policy_library_items, context_setter)| {
      context_setter.set(PolicyLibraryState {
        policy_library_items: policy_library_items.clone(),
      });
    },
  );

  html! {
    <ContextProvider<PolicyLibraryState> context={(*context).clone()}>
      { props.children.clone() }
    </ContextProvider<PolicyLibraryState>>
  }
}

#[hook]
pub fn use_policy_library_context() -> Option<PolicyLibraryState> {
  use_context::<PolicyLibraryState>()
}
