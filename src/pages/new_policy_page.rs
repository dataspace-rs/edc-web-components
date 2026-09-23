use crate::components::CreatePolicy;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct NewPolicyPageProps {
  #[prop_or("New Policy".to_string())]
  pub title: String,
  #[prop_or(Some("Policies define the rules and conditions that govern the use of your data. You can add as many rules as required.".to_string()))]
  pub tag_line: Option<String>,
  pub on_create: Callback<()>,
}

#[component]
pub fn NewPolicyPage(props: &NewPolicyPageProps) -> Html {
  let tag_line = props
    .tag_line
    .as_ref()
    .map(|tag_line| html!(<p>{ tag_line }</p>))
    .unwrap_or_default();

  html!(
    <>
      <Title level={Level::H2} size={Size::XXXLarge}>{ &props.title }</Title>
      { tag_line }
      <CreatePolicy on_create={props.on_create.clone()} />
    </>
  )
}
