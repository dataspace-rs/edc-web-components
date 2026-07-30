use crate::components::{Issuer, RequestVerifiableCredential};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct RequestVerifiableCredentialPageProps {
  #[prop_or_default]
  pub issuers: Vec<Issuer>,
  pub on_create: Callback<()>,
}

#[component]
pub fn RequestVerifiableCredentialPage(props: &RequestVerifiableCredentialPageProps) -> Html {
  html!(
    <Stack gutter=true>
      <StackItem>
        <Title level={Level::H3} size={Size::XXLarge}>{ "Request a Verifiable Credentials" }</Title>
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
