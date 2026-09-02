use crate::components::ShowDataPlane;
use crate::contexts::use_edc_connector_context;
use crate::models::DataPlane;
use edc_connector_client::EdcConnectorApiVersion;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew::suspense::use_future;

#[component]
pub fn DataPlanePage() -> Html {
  html! {
    <Suspense fallback={html!(<Bullseye><Spinner /></Bullseye>)}>
      <DataPlanePageInner />
    </Suspense>
  }
}

#[component]
pub fn DataPlanePageInner() -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();

  let data_planes = use_future(async move || {
    if let Some(edc_connector_client) = edc_connector_context.get_client() {
      edc_connector_client
        .data_planes(EdcConnectorApiVersion::V4)
        .list()
        .await
        .unwrap_or_default()
        .into_iter()
        .map(DataPlane::from)
        .collect::<Vec<DataPlane>>()
    } else {
      Vec::new()
    }
  })?;

  let data_planes = (*data_planes).clone();

  let data_planes = data_planes.into_iter().map(|data_plane| {
    html! { <ShowDataPlane {data_plane} /> }
  });

  Ok(html! { { for data_planes } })
}
