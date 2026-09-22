use crate::components::logo::Logo;
use crate::contexts::{use_did_resolver_context, use_my_did_provider_context};
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct DidLabelProps {
  pub did: String,
}

#[component]
pub fn DidLabel(props: &DidLabelProps) -> Html {
  let did_resolver_context = use_did_resolver_context();
  let my_did_provider_context = use_my_did_provider_context();

  let participant = use_memo(
    (did_resolver_context.clone(), props.did.clone()),
    |(did_resolver_context, did)| {
      did_resolver_context
        .as_ref()
        .and_then(|did_resolver_context| {
          did_resolver_context
            .participant(did)
            .map(|participant| (participant.name.clone(), participant.logo.clone()))
        })
        .unzip()
    },
  );

  let (label, logo) = (*participant).clone();

  let label = label.unwrap_or_else(|| props.did.clone());

  let icon = if my_did_provider_context
    .map(|my_did_provider_context| my_did_provider_context.my_did().to_string())
    == Some(props.did.clone())
  {
    html!(
      <yew_icons::Icon
        data={yew_icons::IconData::LUCIDE_BUILDING}
        style="color: var(--pf-t--global--border--color--default)"
      />
    )
  } else {
    html!()
  };

  html! {
    <Split gutter=true>
      <SplitItem>
        <Logo url={logo.flatten()} width="24px" height="24px" />
      </SplitItem>
      <SplitItem>
        <Tooltip text={props.did.clone()}>
          <Flex>
            <FlexItem modifiers={[FlexModifier::Align(Alignment::Center).all()]}>
              { label }
            </FlexItem>
            <FlexItem modifiers={[FlexModifier::Align(Alignment::Center).all()]}>{ icon }</FlexItem>
          </Flex>
        </Tooltip>
      </SplitItem>
    </Split>
  }
}
