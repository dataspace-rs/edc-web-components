use crate::models::DataPlane;
use edc_connector_client::types::dataplane::DataPlaneInstanceState;
use patternfly_yew::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ShowDataPlaneProps {
  pub data_plane: DataPlane,
}

#[component]
pub fn ShowDataPlane(props: &ShowDataPlaneProps) -> Html {
  let state = match &props.data_plane.state {
    DataPlaneInstanceState::Available => "Available".to_string(),
    DataPlaneInstanceState::Registered => "Registered".to_string(),
    DataPlaneInstanceState::Unavailable => "Unavailable".to_string(),
    DataPlaneInstanceState::Unregistered => "Unregistered".to_string(),
    DataPlaneInstanceState::Other(state) => state.to_string(),
  };

  let allowed_source_types =
    props
      .data_plane
      .allowed_source_types
      .iter()
      .map(|allowed_source_type| {
        html_nested!(
          <FlexItem>
            <Label label={allowed_source_type.to_string()} />
          </FlexItem>
        )
      });

  let allowed_dest_types = props
    .data_plane
    .allowed_dest_types
    .iter()
    .map(|allowed_dest_type| {
      html_nested!(
        <FlexItem>
          <Label label={allowed_dest_type.to_string()} />
        </FlexItem>
      )
    });

  let allowed_transfer_types =
    props
      .data_plane
      .allowed_transfer_types
      .iter()
      .map(|allowed_transfer_type| {
        html_nested!(
          <FlexItem>
            <Label label={allowed_transfer_type.to_string()} />
          </FlexItem>
        )
      });

  html! {
    <Card>
      <CardTitle>{ "Data Plane" }</CardTitle>
      <CardBody>
        <DescriptionList mode={[DescriptionListMode::Horizontal]}>
          <DescriptionGroup term="Id">{ &props.data_plane.id }</DescriptionGroup>
          <DescriptionGroup term="URL">{ &props.data_plane.url }</DescriptionGroup>
          <DescriptionGroup term="Allowed Source Types">
            <Flex>{ for allowed_source_types }</Flex>
          </DescriptionGroup>
          <DescriptionGroup term="Allowed Destination Types">
            <Flex>{ for allowed_dest_types }</Flex>
          </DescriptionGroup>
          <DescriptionGroup term="Allowed Transfer Types">
            <Flex>{ for allowed_transfer_types }</Flex>
          </DescriptionGroup>
          <DescriptionGroup term="State">{ state }</DescriptionGroup>
        </DescriptionList>
      </CardBody>
    </Card>
  }
}
