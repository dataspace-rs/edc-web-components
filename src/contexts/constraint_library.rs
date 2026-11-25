use codes_iso_4217::{ALL_CODES, CurrencyCode};
use edc_connector_client::types::policy::Constraint;
use edc_connector_client::types::properties::PropertyValue;
use serde_json::Value;
use yew::prelude::*;

/// The kind of policy rule a library constraint belongs to.
/// Displayed with its plural label, as used for the policy sections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::Display, strum::EnumIter)]
pub enum RuleKind {
  #[strum(to_string = "Permissions")]
  Permission,
  #[strum(to_string = "Prohibitions")]
  Prohibition,
  #[strum(to_string = "Obligations")]
  Obligation,
}

/// The kind of value a library constraint parameter holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintParameterKind {
  /// An ISO 8601 date time in UTC, stored as a string, e.g. `2027-01-01T00:00:00Z`.
  DateTime,
  /// A non-negative decimal number, stored as a JSON number, e.g. `12.5`.
  Float,
  /// An ISO 4217 currency code, stored as a string, e.g. `EUR`.
  Currency,
}

impl ConstraintParameterKind {
  /// Value used when the constraint is added, before the user changes it.
  pub fn default_value(&self) -> Value {
    match self {
      ConstraintParameterKind::DateTime => {
        let now = String::from(js_sys::Date::new_0().to_iso_string());
        Value::String(from_datetime_local(now.get(..16).unwrap_or_default()))
      }
      ConstraintParameterKind::Float => Value::from(0.0),
      ConstraintParameterKind::Currency => Value::String("EUR".to_string()),
    }
  }
}

/// A value the user provides when adding a library constraint to a policy.
///
/// With a single parameter, its value is the right operand of the constraint. With several,
/// the right operand is a JSON object holding each value under the parameter `key`.
#[derive(Debug, Clone, PartialEq)]
pub struct ConstraintParameter {
  pub key: String,
  pub label: String,
  pub kind: ConstraintParameterKind,
}

impl ConstraintParameter {
  pub fn new(key: &str, label: &str, kind: ConstraintParameterKind) -> Self {
    Self {
      key: key.to_string(),
      label: label.to_string(),
      kind,
    }
  }
}

/// Converts an ISO 8601 UTC date time (`2027-01-01T00:00:00Z`) to the value of a
/// `datetime-local` input (`2027-01-01T00:00`).
pub fn to_datetime_local(value: &str) -> String {
  value.get(..16).unwrap_or(value).to_string()
}

/// Converts the value of a `datetime-local` input (`2027-01-01T00:00`, seconds optional) to
/// an ISO 8601 UTC date time (`2027-01-01T00:00:00Z`).
pub fn from_datetime_local(value: &str) -> String {
  match value.len() {
    0 => String::new(),
    16 => format!("{value}:00Z"),
    _ => format!("{value}Z"),
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstraintLibraryItem {
  pub name: String,
  pub description: Option<String>,
  pub rule_kind: RuleKind,
  /// The constraint added to the policy. With `parameters`, its right operand is replaced by
  /// the values provided by the user.
  pub constraint: Constraint,
  pub parameters: Vec<ConstraintParameter>,
}

impl ConstraintLibraryItem {
  /// Whether `constraint` comes from this library item. With parameters, the right operand
  /// is ignored since it holds the user values.
  pub fn matches(&self, constraint: &Constraint) -> bool {
    if self.parameters.is_empty() {
      return &self.constraint == constraint;
    }

    match (&self.constraint, constraint) {
      (Constraint::Atomic(template), Constraint::Atomic(atomic)) => {
        template.left_operand == atomic.left_operand && template.operator == atomic.operator
      }
      _ => false,
    }
  }

  /// The constraint to add to a policy, with the parameters default values.
  pub fn new_constraint(&self) -> Constraint {
    if self.parameters.is_empty() {
      return self.constraint.clone();
    }

    let values = self
      .parameters
      .iter()
      .map(|parameter| parameter.kind.default_value())
      .collect::<Vec<_>>();

    self.with_parameter_values(&values)
  }

  /// The constraint with `values` (one per parameter, in order) as right operand.
  pub fn with_parameter_values(&self, values: &[Value]) -> Constraint {
    let right_operand = match (self.parameters.as_slice(), values) {
      ([_], [value]) => value.clone(),
      _ => Value::Object(
        self
          .parameters
          .iter()
          .zip(values)
          .map(|(parameter, value)| (parameter.key.clone(), value.clone()))
          .collect(),
      ),
    };

    match &self.constraint {
      Constraint::Atomic(template) => {
        let mut atomic = template.clone();
        atomic.right_operand = PropertyValue(right_operand);
        Constraint::Atomic(atomic)
      }
      constraint => constraint.clone(),
    }
  }

  /// The values held by `constraint`, one per parameter, in order. Missing values are `null`.
  pub fn parameter_values(&self, constraint: &Constraint) -> Vec<Value> {
    let Constraint::Atomic(atomic) = constraint else {
      return vec![Value::Null; self.parameters.len()];
    };

    match (self.parameters.as_slice(), &atomic.right_operand.0) {
      ([_], value) => vec![value.clone()],
      (parameters, Value::Object(object)) => parameters
        .iter()
        .map(|parameter| object.get(&parameter.key).cloned().unwrap_or(Value::Null))
        .collect(),
      (parameters, _) => vec![Value::Null; parameters.len()],
    }
  }
}

/// ISO 4217 currencies offered for a currency parameter, sorted by name. Funds are left out.
///
/// `codes-iso-4217` 0.1.7 merges the historical list into the active one by code, so
/// `is_historical` and `monetary_units` are wrong for currencies that also have a historical
/// entry (e.g. EUR, JPY) and cannot be used to keep active currencies only.
pub fn iso_4217_currencies() -> Vec<CurrencyCode> {
  let mut currencies = ALL_CODES
    .into_iter()
    .filter(|currency| !currency.is_fund())
    .collect::<Vec<_>>();

  currencies.sort_by_key(|currency| currency.currency_name());
  currencies
}

#[derive(Clone, PartialEq)]
pub struct ConstraintLibraryState {
  constraint_library_items: Vec<ConstraintLibraryItem>,
}

impl ConstraintLibraryState {
  pub fn constraint_library(&self) -> &[ConstraintLibraryItem] {
    &self.constraint_library_items
  }
}

#[derive(Properties, PartialEq)]
pub struct ConstraintLibraryContextProviderProps {
  #[prop_or_default]
  pub children: Html,
  pub constraint_library_items: Vec<ConstraintLibraryItem>,
}

#[component]
pub fn ConstraintLibraryContextProvider(props: &ConstraintLibraryContextProviderProps) -> Html {
  let context = use_state(move || ConstraintLibraryState {
    constraint_library_items: props.constraint_library_items.clone(),
  });

  use_effect_with(
    (props.constraint_library_items.clone(), context.setter()),
    |(constraint_library_items, context_setter)| {
      context_setter.set(ConstraintLibraryState {
        constraint_library_items: constraint_library_items.clone(),
      });
    },
  );

  html! {
    <ContextProvider<ConstraintLibraryState> context={(*context).clone()}>
      { props.children.clone() }
    </ContextProvider<ConstraintLibraryState>>
  }
}

#[hook]
pub fn use_constraint_library_context() -> Option<ConstraintLibraryState> {
  use_context::<ConstraintLibraryState>()
}
