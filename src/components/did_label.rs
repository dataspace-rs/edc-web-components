use crate::components::logo::Logo;
use crate::contexts::use_did_resolver_context;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct DidLabelProps {
  pub did: String,
}

#[component]
pub fn DidLabel(props: &DidLabelProps) -> Html {
  let did_resolver_context = use_did_resolver_context();

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

  html! {
    <Split gutter=true>
      <SplitItem>
        <Logo url={logo.flatten()} width="24px" height="24px" />
      </SplitItem>
      <SplitItem>
        <Tooltip text={props.did.clone()}>{ label }</Tooltip>
      </SplitItem>
    </Split>
  }
}
