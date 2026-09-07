use patternfly_yew::prelude::*;
use serde_json::Value;
use std::collections::HashMap;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ExtensiblePropertiesEditProps {
  pub values: HashMap<String, Value>,
  pub onchange: Callback<HashMap<String, Value>>,
}

#[component]
pub fn ExtensiblePropertiesEdit(props: &ExtensiblePropertiesEditProps) -> Html {
  let onclick = use_callback(
    (props.values.clone(), props.onchange.clone()),
    |_, (values, onchange)| {
      let mut values = values.clone();
      values.insert("".to_string(), Value::Null);
      onchange.emit(values);
    },
  );

  let onchange_key = use_callback(
    (props.values.clone(), props.onchange.clone()),
    |(old_key, key), (values, onchange)| {
      let mut values = values.clone();

      if let Some(value) = values.remove(&old_key) {
        values.insert(key, value);
        onchange.emit(values);
      }
    },
  );

  let onchange_value = use_callback(
    (props.values.clone(), props.onchange.clone()),
    |(key, value): (String, String), (values, onchange)| {
      let mut values = values.clone();

      let value = serde_json::from_str(&value).unwrap_or_default();

      values.insert(key, value);
      onchange.emit(values);
    },
  );

  let items = props.values.iter().map(|(key, value)| {
    let old_key = key.to_string();
    let key = key.to_string();

    let serde_json_value = serde_json::to_string_pretty(value).unwrap_or_default();

    html_nested!(
      <>
        <GridItem cols={[4]}>
          <FormGroup label="Key" required=true>
            <TextInput
              required=true
              value={key.to_string()}
              onchange={onchange_key.reform(move |key| (old_key.clone(), key))}
            />
          </FormGroup>
        </GridItem>
        <GridItem cols={[8]}>
          <FormGroup
            label="Value"
            label_icon={LabelIcon::Help(html_nested!(<PopoverBody>{ "Enter a valid json." } </PopoverBody>))}
          >
            <TextInput
              required=true
              value={serde_json_value}
              onchange={onchange_value.reform(move |value| (key.clone(), value))}
            />
          </FormGroup>
        </GridItem>
      </>
    )
  });

  html!(
    <Stack gutter=true>
      <StackItem>
        <Button variant={ButtonVariant::Primary} icon={Icon::Plus} {onclick}>{ "Add" }</Button>
      </StackItem>
      <StackItem>
        <Grid gutter=true>{ for items }</Grid>
      </StackItem>
    </Stack>
  )
}
