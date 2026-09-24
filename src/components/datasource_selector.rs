use crate::components::one_state_selector::OneStateSelector;
use crate::contexts::use_edc_connector_context;
use edc_connector_client::EdcConnectorApiVersion;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct DatasourceSelectorProps {
  pub selected_kind: String,
  pub on_selected_kind: Callback<String>,
}

#[component]
pub fn DatasourceSelector(props: &DatasourceSelectorProps) -> Html {
  html! {
    <Suspense fallback={html!(<Bullseye><Spinner /></Bullseye>)}>
      <DatasourceSelectorInner
        selected_kind={props.selected_kind.clone()}
        on_selected_kind={props.on_selected_kind.clone()}
      />
    </Suspense>
  }
}

#[component]
pub fn DatasourceSelectorInner(props: &DatasourceSelectorProps) -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();

  let datasource_types = use_future_with(
    edc_connector_context.clone(),
    async |edc_connector_context| {
      if let Some(edc_connector_client) = edc_connector_context.get_client() {
        let mut datasource_types = edc_connector_client
          .data_planes(EdcConnectorApiVersion::V4)
          .list()
          .await
          .unwrap_or_default()
          .into_iter()
          .flat_map(|dataplane| dataplane.allowed_source_types().to_vec())
          .collect::<Vec<String>>();

        datasource_types.dedup();
        datasource_types.sort();

        datasource_types
      } else {
        vec![]
      }
    },
  )?;

  let items = (*datasource_types).clone();

  Ok(html!(
    <OneStateSelector
      selectable_items={items}
      selected_item={props.selected_kind.clone()}
      on_selected={props.on_selected_kind.clone()}
      default_label="Select a Data Source Type"
    />
  ))
}
