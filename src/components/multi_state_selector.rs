use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct MultiStateSelectorProps {
  pub selectable_items: Vec<(String, bool)>,
  pub on_selected: Callback<Vec<(String, bool)>>,
  #[prop_or("All".to_string())]
  pub none_selected_label: String,
  #[prop_or("All".to_string())]
  pub all_selected_label: String,
}

#[component]
pub fn MultiStateSelector(props: &MultiStateSelectorProps) -> Html {
  let onclick = use_callback(
    (props.selectable_items.clone(), props.on_selected.clone()),
    |index, (selectable_items, on_selected)| {
      let mut selectable_items = selectable_items.clone();

      let item: &mut (String, bool) = selectable_items.get_mut(index).unwrap();
      item.1 = !item.1;

      on_selected.emit(selectable_items.clone());
    },
  );

  let actions = props
    .selectable_items
    .iter()
    .enumerate()
    .map(|(index, (label, selected))| {
      html_nested!(
        <MenuAction onclick={onclick.reform(move |_| index)} {selected}>{ &label }</MenuAction>
      )
    });

  let all_selected = props.selectable_items.iter().all(|(_, selected)| *selected);
  let none_selected = props
    .selectable_items
    .iter()
    .all(|(_, selected)| !*selected);

  let text = if all_selected || none_selected {
    if none_selected {
      props.none_selected_label.clone()
    } else {
      props.all_selected_label.clone()
    }
  } else {
    props
      .selectable_items
      .iter()
      .filter_map(|(label, selected)| if *selected { Some(label.clone()) } else { None })
      .collect::<Vec<String>>()
      .join(", ")
  };

  html!(<Dropdown {text}>{ for actions }</Dropdown>)
}
