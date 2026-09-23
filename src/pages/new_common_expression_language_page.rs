use crate::components::CreateCommonExpressionLanguage;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct NewCommonExpressionLanguagePageProps {
  #[prop_or("New Common Expression Language".to_string())]
  pub title: String,
  #[prop_or(None)]
  pub tag_line: Option<String>,
  pub on_create: Callback<()>,
}

#[component]
pub fn NewCommonExpressionLanguagePage(props: &NewCommonExpressionLanguagePageProps) -> Html {
  let tag_line = props
    .tag_line
    .as_ref()
    .map(|tag_line| html!(<p>{ tag_line }</p>))
    .unwrap_or_default();

  html!(
    <>
      <Title level={Level::H2} size={Size::XXXLarge}>{ &props.title }</Title>
      { tag_line }
      <CreateCommonExpressionLanguage on_create={props.on_create.clone()} />
    </>
  )
}
