use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct CelExpressionLibraryItem {
  pub name: String,
  pub left_operand: String,
  pub description: Option<String>,
  pub scopes: Vec<String>,
  pub expression: String,
}

#[derive(Clone, PartialEq)]
pub struct CelExpressionLibraryState {
  cel_expression_library_items: Vec<CelExpressionLibraryItem>,
}

impl CelExpressionLibraryState {
  pub fn cel_expression_library(&self) -> &[CelExpressionLibraryItem] {
    &self.cel_expression_library_items
  }
}

#[derive(Properties, PartialEq)]
pub struct CelExpressionLibraryContextProviderProps {
  #[prop_or_default]
  pub children: Html,
  pub cel_expression_library_items: Vec<CelExpressionLibraryItem>,
}

#[component]
pub fn CelExpressionLibraryContextProvider(
  props: &CelExpressionLibraryContextProviderProps,
) -> Html {
  let context = use_state(move || CelExpressionLibraryState {
    cel_expression_library_items: props.cel_expression_library_items.clone(),
  });

  use_effect_with(
    (props.cel_expression_library_items.clone(), context.setter()),
    |(cel_expression_library_items, context_setter)| {
      context_setter.set(CelExpressionLibraryState {
        cel_expression_library_items: cel_expression_library_items.clone(),
      });
    },
  );

  html! {
    <ContextProvider<CelExpressionLibraryState> context={(*context).clone()}>
      { props.children.clone() }
    </ContextProvider<CelExpressionLibraryState>>
  }
}

#[hook]
pub fn use_cel_expression_library_context() -> Option<CelExpressionLibraryState> {
  use_context::<CelExpressionLibraryState>()
}
