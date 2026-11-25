use crate::contexts::{
  ConstraintParameter, ConstraintParameterKind, from_datetime_local, iso_4217_currencies,
  to_datetime_local,
};
use patternfly_yew::prelude::*;
use serde_json::Value;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ConstraintParameterInputProps {
  pub parameter: ConstraintParameter,
  pub value: Value,
  pub onchange: Callback<Value>,
}

/// Edits the value of a library constraint parameter, with the input matching its kind.
/// The parameter label is shown as helper text below the input.
#[component]
pub fn ConstraintParameterInput(props: &ConstraintParameterInputProps) -> Html {
  let value = match &props.value {
    Value::String(value) => value.clone(),
    Value::Null => String::new(),
    value => value.to_string(),
  };

  let input = match props.parameter.kind {
    ConstraintParameterKind::DateTime => {
      let onchange = props
        .onchange
        .reform(|value: String| Value::String(from_datetime_local(&value)));

      html!(
        <TextInput
          r#type={TextInputType::DateTimeLocal}
          value={to_datetime_local(&value)}
          required=true
          {onchange}
        />
      )
    }
    ConstraintParameterKind::Float => html!(
      <FloatParameterInput value={props.value.clone()} onchange={props.onchange.clone()} />
    ),
    ConstraintParameterKind::Currency => {
      let onchange = props
        .onchange
        .reform(|code: Option<String>| code.map(Value::String).unwrap_or(Value::Null));

      let options = iso_4217_currencies().into_iter().map(|currency| {
        let code = currency.alpha_code();

        html_nested!(
          <FormSelectOption<String>
            value={code.to_string()}
            description={format!("{} ({code})", currency.currency_name())}
            selected={code == value}
          />
        )
      });

      html!(
        <FormSelect<String> value={Some(value.clone())} required=true {onchange}>
          { for options }
        </FormSelect<String>>
      )
    }
  };

  html!(
    <>
      { input }
      <HelperText>
        <HelperTextItem>{ &props.parameter.label }</HelperTextItem>
      </HelperText>
    </>
  )
}

/// Parses a non-negative decimal number, accepting a comma as decimal separator.
fn parse_float(value: &str) -> Option<f64> {
  value
    .trim()
    .replace(',', ".")
    .parse::<f64>()
    .ok()
    .filter(|value| value.is_finite() && *value >= 0.0)
}

#[derive(Clone, PartialEq, Properties)]
struct FloatParameterInputProps {
  value: Value,
  onchange: Callback<Value>,
}

/// Decimal input keeping the text as typed: re-rendering it from the parsed number while
/// typing would turn `1` into `1.0` and corrupt the next keystrokes.
#[component]
fn FloatParameterInput(props: &FloatParameterInputProps) -> Html {
  let display = |value: &Value| match value {
    Value::String(value) => value.clone(),
    Value::Null => String::new(),
    value => value.to_string(),
  };

  let text = use_state_eq(|| display(&props.value));

  // Follow outside changes of the value, but not the ones coming from this input.
  {
    let text = text.clone();
    use_effect_with(props.value.clone(), move |value| {
      let typed = parse_float(&text);
      let current = value
        .as_f64()
        .or_else(|| value.as_str().and_then(parse_float));

      if typed != current || (current.is_none() && *text != display(value)) {
        text.set(display(value));
      }
    });
  }

  let onchange = use_callback(
    (text.setter(), props.onchange.clone()),
    |value: String, (text_setter, onchange)| {
      text_setter.set(value.clone());

      // Keep what was typed when it is not a valid number, so the user can fix it.
      onchange.emit(
        parse_float(&value)
          .and_then(serde_json::Number::from_f64)
          .map(Value::Number)
          .unwrap_or(Value::String(value)),
      );
    },
  );

  let state = if parse_float(&text).is_some() {
    InputState::Default
  } else {
    InputState::Error
  };

  // A text input: a number input only accepts integers unless a `step` is set.
  html!(<TextInput value={(*text).clone()} inputmode="decimal" required=true {state} {onchange} />)
}
