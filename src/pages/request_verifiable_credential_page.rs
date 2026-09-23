use crate::components::{Issuer, RequestVerifiableCredential};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct RequestVerifiableCredentialPageProps {
  #[prop_or("Request a Verifiable Credentials".to_string())]
  pub title: String,
  #[prop_or(None)]
  pub tag_line: Option<String>,
  #[prop_or_default]
  pub issuers: Vec<Issuer>,
  pub on_create: Callback<()>,
}

#[component]
pub fn RequestVerifiableCredentialPage(props: &RequestVerifiableCredentialPageProps) -> Html {
  let tag_line = props
    .tag_line
    .as_ref()
    .map(|tag_line| html!(<p>{ tag_line }</p>))
    .unwrap_or_default();

  html!(
    <Stack gutter=true>
      <StackItem>
        <Title level={Level::H3} size={Size::XXLarge}>{ &props.title }</Title>
        { tag_line }
      </StackItem>
      <StackItem>
        <RequestVerifiableCredential
          on_create={props.on_create.clone()}
          issuers={props.issuers.clone()}
        />
      </StackItem>
    </Stack>
  )
}
