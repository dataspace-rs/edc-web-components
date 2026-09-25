use edc_connector_client::types::policy::Constraint;
use std::{ops::Deref, rc::Rc};
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct PolicyConstraintRenderer {
  pub action: String,
  pub constraint: Constraint,
  pub title: String,
  pub inner: Html,
}

impl PolicyConstraintRenderer {
  fn is(&self, action: &str, constraint: &Constraint) -> bool {
    self.action == action && &self.constraint == constraint
  }
}

#[derive(Clone, PartialEq)]
pub struct PolicyConstraintRendererState {
  constraints: Vec<PolicyConstraintRenderer>,
}

impl PolicyConstraintRendererState {
  pub fn renderer_constraint(
    &self,
    action: &str,
    constraint: &Constraint,
  ) -> Option<(String, Html)> {
    self
      .constraints
      .iter()
      .find(|policy_constraint_renderer| policy_constraint_renderer.is(action, constraint))
      .map(|policy_constraint_renderer| {
        (
          policy_constraint_renderer.title.to_string(),
          policy_constraint_renderer.inner.clone(),
        )
      })
  }
}

impl Reducible for PolicyConstraintRendererState {
  type Action = ();

  fn reduce(self: Rc<Self>, _action: Self::Action) -> Rc<Self> {
    let new_self = self.deref().clone();
    new_self.into()
  }
}

#[derive(Properties, PartialEq)]
pub struct PolicyConstraintRendererContextProviderProps {
  #[prop_or_default]
  pub children: Html,
  pub constraints: Vec<PolicyConstraintRenderer>,
}

#[component]
pub fn PolicyConstraintRendererContextProvider(
  props: &PolicyConstraintRendererContextProviderProps,
) -> Html {
  let context = use_reducer(move || PolicyConstraintRendererState {
    constraints: props.constraints.clone(),
  });

  html! {
    <ContextProvider<PolicyConstraintRendererContext> {context}>
      { props.children.clone() }
    </ContextProvider<PolicyConstraintRendererContext>>
  }
}

pub type PolicyConstraintRendererContext = UseReducerHandle<PolicyConstraintRendererState>;

#[hook]
pub fn use_policy_constraint_renderer_context() -> Option<PolicyConstraintRendererContext> {
  use_context::<PolicyConstraintRendererContext>()
}
