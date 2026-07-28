use crate::components::RequestVerifiableCredential;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct RequestVerifiableCredentialPageProps {
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
        <RequestVerifiableCredential on_create={props.on_create.clone()} />
      </StackItem>
    </Stack>
  )
}
