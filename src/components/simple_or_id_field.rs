use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
  pub is_simple: bool,
  pub value: String,
  pub onchange: Callback<(bool, String)>,
}

#[component]
pub fn SimpleOrIdField(props: &Props) -> Html {
  let is_simple_target = use_state(|| props.is_simple);
  let target = use_state(|| props.value.clone());

  let on_target_id = use_callback(
    (
      props.onchange.clone(),
      is_simple_target.clone(),
      target.clone(),
    ),
    move |_, (onchange, is_simple_target, target)| {
      is_simple_target.set(false);
      onchange.emit((false, (**target).to_string()))
    },
  );

  let on_target_simple = use_callback(
    (
      props.onchange.clone(),
      is_simple_target.clone(),
      target.clone(),
    ),
    move |_, (onchange, is_simple_target, target)| {
      is_simple_target.set(true);
      onchange.emit((true, (**target).to_string()))
    },
  );

  let oninput_target = use_callback(
    (
      props.onchange.clone(),
      is_simple_target.clone(),
      target.clone(),
    ),
    move |value: String, (onchange, is_simple_target, target)| {
      target.set(value.clone());
      onchange.emit((**is_simple_target, value))
    },
  );

  let target_mode = if *is_simple_target { "Simple" } else { "ID" };

  html!(
    <Split>
      <SplitItem>
        <Dropdown text={html!(target_mode)}>
          <MenuAction onclick={on_target_simple}>{ "Simple" }</MenuAction>
          <MenuAction onclick={on_target_id}>{ "ID" }</MenuAction>
        </Dropdown>
      </SplitItem>
      <SplitItem fill=true>
        <TextInput value={(*target).clone()} onchange={oninput_target} />
      </SplitItem>
    </Split>
  )
}
