use crate::contexts::use_edc_connector_context;
use edc_connector_client::EdcConnectorApiVersion;
use edc_connector_client::types::common_expression_language::CommonExpressionLanguage;
use edc_connector_client::types::query::Query;
use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, PartialEq, Properties)]
pub struct CelAtomicConstraintEditProps {
  pub cel_left_operand: String,
  pub on_change: Callback<String>,
}

#[component]
pub fn CelAtomicConstraintEdit(props: &CelAtomicConstraintEditProps) -> Html {
  html!(
    <Suspense fallback="Loading...">
      <CelSelector
        cel_left_operand={props.cel_left_operand.clone()}
        on_change={props.on_change.clone()}
      />
    </Suspense>
  )
}

#[derive(Clone, PartialEq, Properties)]
pub struct CelSelectorProps {
  pub cel_left_operand: String,
  pub on_change: Callback<String>,
}

#[component]
pub fn CelSelector(props: &CelSelectorProps) -> HtmlResult {
  let edc_connector_context = use_edc_connector_context();

  let cel_list = use_future_with(edc_connector_context.clone(), |parameters| async move {
    let edc_connector_context = (*parameters).clone();

    let query = Query::builder().build();

    if let Some(client) = edc_connector_context.get_client() {
      client
        .common_expression_language(EdcConnectorApiVersion::V5Beta)
        .query(query)
        .await
        .map_err(|error| {
          log::error!("Error: {}", error);
          error
        })
        .unwrap_or_default()
        .into_iter()
        .map(CommonExpressionLanguageItem::from)
        .collect::<Vec<_>>()
    } else {
      vec![]
    }
  })?;

  let cel_list = (*cel_list).clone();

  if props.cel_left_operand.is_empty()
    && let Some(first_cel) = cel_list.first()
  {
    props.on_change.emit(first_cel.left_operand.clone());
  }

  let onclick = use_callback(props.on_change.clone(), move |left_operand, on_change| {
    on_change.emit(left_operand);
  });

  let items = cel_list.iter().map(|cel| {
    let left_operand = cel.left_operand.clone();

    html_nested!(
      <MenuAction
        selected={props.cel_left_operand == cel.left_operand}
        onclick={onclick.reform(move |_| left_operand.clone())}
      >
        <DescriptionList>
          <DescriptionGroup term={cel.left_operand.clone()}>
            <Truncate
              style="max-width: 50vw;"
              content={cel.description.clone().unwrap_or_default()}
            />
          </DescriptionGroup>
        </DescriptionList>
      </MenuAction>
    )
  });

  Ok(html!(
    <Flex>
      <FlexItem>
        <Dropdown text={props.cel_left_operand.clone()}>{ for items }</Dropdown>
      </FlexItem>
    </Flex>
  ))
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommonExpressionLanguageItem {
  pub left_operand: String,
  pub description: Option<String>,
}
impl From<CommonExpressionLanguage> for CommonExpressionLanguageItem {
  fn from(value: CommonExpressionLanguage) -> Self {
    CommonExpressionLanguageItem {
      left_operand: value.left_operand().to_string(),
      description: value.description().clone(),
    }
  }
}
