use crate::contexts::ConstraintLibraryItem;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ConstraintLibrarySearchProps {
  /// Library constraints offered in the dropdown menu.
  pub items: Vec<ConstraintLibraryItem>,
  /// Emitted when a constraint is picked in the dropdown menu.
  pub onselect: Callback<ConstraintLibraryItem>,
  #[prop_or("Search a constraint to add".to_string())]
  pub placeholder: String,
}

/// Search input with a dropdown menu listing the matching library constraints (name and
/// description). Picking one emits it through `onselect` and clears the search.
///
/// It can be used inside a form: Enter does not submit it.
#[component]
pub fn ConstraintLibrarySearch(props: &ConstraintLibrarySearchProps) -> Html {
  let search = use_state_eq(String::new);
  let menu_open = use_state_eq(|| false);

  let onsearch = use_callback(
    (search.setter(), menu_open.setter()),
    |value: String, (search_setter, menu_open_setter)| {
      search_setter.set(value);
      menu_open_setter.set(true);
    },
  );

  let onclear = use_callback(search.setter(), |_, search_setter| {
    search_setter.set(String::new());
  });

  let onfocusin = use_callback(menu_open.setter(), |_: FocusEvent, menu_open_setter| {
    menu_open_setter.set(true);
  });

  let onfocusout = use_callback(menu_open.setter(), |_: FocusEvent, menu_open_setter| {
    menu_open_setter.set(false);
  });

  // Enter must not submit the surrounding form, Escape closes the menu.
  let onkeydown = use_callback(
    menu_open.setter(),
    |event: KeyboardEvent, menu_open_setter| match event.key().as_str() {
      "Enter" => event.prevent_default(),
      "Escape" => menu_open_setter.set(false),
      _ => {}
    },
  );

  // Keep the focus in the search input while clicking a choice, so the menu stays open.
  let onmousedown_menu = Callback::from(|event: MouseEvent| event.prevent_default());

  let onpick = use_callback(
    (props.onselect.clone(), search.setter()),
    |item: ConstraintLibraryItem, (onselect, search_setter)| {
      onselect.emit(item);
      search_setter.set(String::new());
    },
  );

  let search_value = search.trim().to_lowercase();

  let choices = props
    .items
    .iter()
    .filter(|item| {
      search_value.is_empty()
        || item.name.to_lowercase().contains(&search_value)
        || item
          .description
          .as_ref()
          .is_some_and(|description| description.to_lowercase().contains(&search_value))
    })
    .map(|item| {
      let item = item.clone();
      let name = item.name.clone();

      html_nested!(
        <MenuAction
          description={item.description.clone()}
          onclick={onpick.reform(move |_| item.clone())}
        >
          { name }
        </MenuAction>
      )
    })
    .collect::<Vec<_>>();

  // The menu container is always rendered and only hidden: inserting it next to the
  // SearchInput would make Yew recreate the input, which loses the focus.
  let menu_hidden = !*menu_open || choices.is_empty();

  html!(
    <div style="position: relative" {onfocusin} {onfocusout} {onkeydown}>
      <SearchInput
        value={(*search).clone()}
        placeholder={props.placeholder.clone()}
        onchange={onsearch}
        {onclear}
      />
      <div
        style="position: absolute; top: 100%; left: 0; right: 0; z-index: 1000"
        hidden={menu_hidden}
        onmousedown={onmousedown_menu}
      >
        <Menu scrollable=true>{ for choices }</Menu>
      </div>
    </div>
  )
}
