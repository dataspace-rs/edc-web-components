use edc_connector_client::types::common_expression_language::CommonExpressionLanguage;

#[derive(Clone, Debug, PartialEq)]
pub struct CommonExpressionLanguageItem {
  pub id: String,
  pub left_operand: String,
  pub description: Option<String>,
  pub scopes: Vec<String>,
  pub expression: String,
}

impl From<CommonExpressionLanguage> for CommonExpressionLanguageItem {
  fn from(common_expression_language: CommonExpressionLanguage) -> Self {
    let id = common_expression_language.id().to_string();
    let left_operand = common_expression_language.left_operand().to_string();
    let description = common_expression_language.description().clone();
    let scopes = common_expression_language.scopes().to_vec();
    let expression = common_expression_language.expression().to_string();

    Self {
      id,
      left_operand,
      description,
      scopes,
      expression,
    }
  }
}
