use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct OneStateSelectorProps {
  pub selectable_items: Vec<(String, bool)>,
  pub on_selected: Callback<Vec<(String, bool)>>,
}

#[component]
pub fn OneStateSelector(props: &OneStateSelectorProps) -> Html {
  let onclick = use_callback(
    (props.selectable_items.clone(), props.on_selected.clone()),
    |index, (selectable_items, on_selected)| {
      let mut selectable_items = selectable_items.clone();

      selectable_items
        .iter_mut()
        .enumerate()
        .for_each(|(ind, (_, selected))| {
          if ind == index {
            *selected = !*selected;
          } else {
            *selected = false;
          }
        });

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

  let text = if !props.selectable_items.iter().any(|(_, selected)| *selected) {
    "Select an option".to_string()
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
