use crate::components::DidLabel;
use crate::contexts::use_edc_identity_hub_context;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ShowVerifiableCredentialPageProps {
  #[prop_or("Verifiable Credential".to_string())]
  pub title: String,
  #[prop_or(None)]
  pub tag_line: Option<String>,
  pub verifiable_credential_id: String,
}

#[component]
pub fn ShowVerifiableCredentialPage(props: &ShowVerifiableCredentialPageProps) -> Html {
  let tag_line = props
    .tag_line
    .as_ref()
    .map(|tag_line| html!(<p>{ tag_line }</p>))
    .unwrap_or_default();

  html!(
    <>
      <Title level={Level::H2} size={Size::XXXLarge}>{ &props.title }</Title>
      { tag_line }
      <Suspense fallback={html! {<Bullseye><Spinner /></Bullseye>}}>
        <ShowVerifiableCredentialPageInner
          verifiable_credential_id={props.verifiable_credential_id.clone()}
        />
      </Suspense>
    </>
  )
}

#[component]
pub fn ShowVerifiableCredentialPageInner(props: &ShowVerifiableCredentialPageProps) -> HtmlResult {
  let edc_identity_hub_context = use_edc_identity_hub_context();

  let verifiable_credential = use_future_with(
    (
      props.verifiable_credential_id.clone(),
      edc_identity_hub_context.clone(),
    ),
    |properties| async move {
      let (verifiable_credential_id, edc_identity_hub_context) = (*properties).clone();
      edc_identity_hub_context
        .get_client()
        .get_credential(
          edc_identity_hub_context.participant_id(),
          &verifiable_credential_id,
        )
        .await
        .ok()
    },
  )?;

  let verifiable_credential = (*verifiable_credential).clone();

  if let Some(verifiable_credential) = verifiable_credential {
    let types = verifiable_credential
      .verifiable_credential
      .credential
      .r#type
      .iter()
      .map(|type_name| {
        html_nested!(
          <FlexItem>
            <Label label={type_name.to_string()} color={Color::Blue} />
          </FlexItem>
        )
      });

    let credential_subjects = verifiable_credential
      .verifiable_credential
      .credential
      .credential_subject
      .iter()
      .map(|credential_subject| {
        html!(
          <CodeBlock>
            <CodeBlockCode>
              { serde_json::to_string_pretty(credential_subject).unwrap_or_default() }
            </CodeBlockCode>
          </CodeBlock>
        )
      });

    Ok(html!(
      <DescriptionList mode={[DescriptionListMode::Horizontal]}>
        <DescriptionGroup term="Id">{ verifiable_credential.id }</DescriptionGroup>
        <DescriptionGroup term="Issuer">
          <DidLabel did={verifiable_credential.issuer_id} />
        </DescriptionGroup>
        <DescriptionGroup term="Holder">
          <DidLabel did={verifiable_credential.holder_id} />
        </DescriptionGroup>
        <DescriptionGroup term="Insurance Date">
          { verifiable_credential.verifiable_credential.credential.issuance_date.format("%Y-%m-%d %H:%M:%S").to_string() }
        </DescriptionGroup>
        <DescriptionGroup term="Expiration Date">
          { verifiable_credential.verifiable_credential.credential.expiration_date.format("%Y-%m-%d %H:%M:%S").to_string() }
        </DescriptionGroup>
        <DescriptionGroup term="Name">
          <DidLabel did={verifiable_credential.verifiable_credential.credential.name} />
        </DescriptionGroup>
        <DescriptionGroup term="Description">
          { verifiable_credential.verifiable_credential.credential.description.unwrap_or_default() }
        </DescriptionGroup>
        <DescriptionGroup term="Types">
          <Flex>{ for types }</Flex>
        </DescriptionGroup>
        <DescriptionGroup term="Credential Subjects">{ for credential_subjects }</DescriptionGroup>
      </DescriptionList>
    ))
  } else {
    Ok(html!(
      format!(
      "Verifiable Credential with id {} not found.",
      props.verifiable_credential_id
    )
    ))
  }
}
