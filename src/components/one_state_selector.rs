use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct OneStateSelectorProps {
  pub selectable_items: Vec<String>,
  pub selected_item: Option<String>,
  pub on_selected: Callback<String>,
  #[prop_or("Select an option".to_string())]
  pub default_label: String,
}

#[component]
pub fn OneStateSelector(props: &OneStateSelectorProps) -> Html {
  let onclick = use_callback(
    (props.selectable_items.clone(), props.on_selected.clone()),
    |index, (selectable, on_selected)| {
      let selected = selectable.get(index).cloned().unwrap_or_default();

      on_selected.emit(selected);
    },
  );

  let actions = props
    .selectable_items
    .iter()
    .enumerate()
    .map(|(index, label)| {
      let selected = label.clone() == props.selected_item.clone().unwrap().as_str();
      html_nested!(
        <MenuAction onclick={onclick.reform(move |_| index)} {selected}>{ &label }</MenuAction>
      )
    });

  let text = if let Some(value) = props.selected_item.clone()
    && !value.is_empty()
  {
    value
  } else {
    props.default_label.clone()
  };

  html!(<Dropdown {text}>{ for actions }</Dropdown>)
}
