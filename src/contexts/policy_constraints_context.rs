use edc_connector_client::types::policy::Constraint;
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
  let context = use_state(move || PolicyConstraintRendererState {
    constraints: props.constraints.clone(),
  });

  use_effect_with(
    (props.constraints.clone(), context.setter()),
    |(constraints, context_setter)| {
      context_setter.set(PolicyConstraintRendererState {
        constraints: constraints.clone(),
      });
    },
  );

  html! {
    <ContextProvider<PolicyConstraintRendererState> context={(*context).clone()}>
      { props.children.clone() }
    </ContextProvider<PolicyConstraintRendererState>>
  }
}

#[hook]
pub fn use_policy_constraint_renderer_context() -> Option<PolicyConstraintRendererState> {
  use_context::<PolicyConstraintRendererState>()
}
