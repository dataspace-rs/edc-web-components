use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct IdentifierProps {
  pub id: String,
}

#[component]
pub fn Identifier(props: &IdentifierProps) -> Html {
  html!(
    <div class="pf-v6-u-font-family-monospace">
      <small>{ &props.id }</small>
    </div>
  )
}
