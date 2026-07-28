use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct StringListEditProps {
  pub values: Vec<String>,
  pub onchange: Callback<Vec<String>>,
  #[prop_or("Add".to_string())]
  pub add_button_label: String,
}

#[component]
pub fn StringListEdit(props: &StringListEditProps) -> Html {
  let onchange = use_callback(
    (props.values.clone(), props.onchange.clone()),
    |(index, value), (values, onchange)| {
      let mut values = values.clone();
      values[index] = value;
      onchange.emit(values);
    },
  );

  let ondelete = use_callback(
    (props.values.clone(), props.onchange.clone()),
    |index, (values, onchange)| {
      let mut values = values.clone();
      values.remove(index);
      onchange.emit(values);
    },
  );

  let inputs = props.values.iter().enumerate().map(|(index, value)| {
    html! {
      <StackItem>
        <Split gutter=true>
          <SplitItem>
            <TextInput
              value={value.to_string()}
              onchange={onchange.reform(move |value| (index, value))}
            />
          </SplitItem>
          <SplitItem>
            <Button
              variant={ButtonVariant::Danger}
              icon={Icon::Trash}
              onclick={ondelete.reform(move |_| index)}
            />
          </SplitItem>
        </Split>
      </StackItem>
    }
  });

  let onclick = use_callback(
    (props.values.clone(), props.onchange.clone()),
    |_, (values, onchange)| {
      let mut values = values.clone();
      values.push("".to_string());
      onchange.emit(values);
    },
  );

  html! {
    <Stack gutter=true>
      <StackItem>
        <Button variant={ButtonVariant::Primary} icon={Icon::Plus} {onclick}>
          { &props.add_button_label }
        </Button>
      </StackItem>
      { for inputs }
    </Stack>
  }
}
