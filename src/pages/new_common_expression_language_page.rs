use crate::components::CreateCommonExpressionLanguage;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct NewCommonExpressionLanguagePageProps {
  pub on_create: Callback<()>,
}

#[component]
pub fn NewCommonExpressionLanguagePage(props: &NewCommonExpressionLanguagePageProps) -> Html {
  html!(
    <>
      <Title level={Level::H2} size={Size::XXXLarge}>{ "New Common Expression Language" }</Title>
      <CreateCommonExpressionLanguage on_create={props.on_create.clone()} />
    </>
  )
}
