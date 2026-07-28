use crate::components::StringListEdit;
use crate::contexts::use_edc_connector_context;
use edc_connector_client::EdcConnectorApiVersion;
use edc_connector_client::types::common_expression_language::NewCommonExpressionLanguage;
use patternfly_yew::prelude::*;
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CreateCommonExpressionLanguageProps {
  #[prop_or_default]
  pub on_create: Callback<()>,
}

#[component]
pub fn CreateCommonExpressionLanguage(props: &CreateCommonExpressionLanguageProps) -> Html {
  let edc_connector_context = use_edc_connector_context();

  let identifier = use_state(|| "".to_string());
  let left_operand = use_state(|| "".to_string());
  let catalog_scope = use_state(|| false);
  let contract_negotiation_scope = use_state(|| false);
  let transfer_process_scope = use_state(|| false);
  let scopes = use_state(Vec::new);
  let expression = use_state(|| "".to_string());
  let description = use_state(|| "".to_string());

  let onchange_identifier = use_callback(identifier.setter(), |value, identifier_setter| {
    identifier_setter.set(value);
  });

  let onchange_left_operand = use_callback(left_operand.setter(), |value, left_operand_setter| {
    left_operand_setter.set(value);
  });

  let onchange_scopes = use_callback(scopes.setter(), |value: Vec<String>, scopes_setter| {
    scopes_setter.set(value);
  });

  let onchange_expression =
    use_callback(expression.setter(), |value: String, expression_setter| {
      expression_setter.set(value);
    });

  let onchange_description = use_callback(description.setter(), |value, description_setter| {
    description_setter.set(value);
  });

  let onsubmit = use_callback(
    (
      edc_connector_context.clone(),
      identifier.clone(),
      left_operand.clone(),
      catalog_scope.clone(),
      contract_negotiation_scope.clone(),
      transfer_process_scope.clone(),
      scopes.clone(),
      expression.clone(),
      description.clone(),
      props.on_create.clone(),
    ),
    |event: SubmitEvent,
     (
      edc_connector_context,
      identifier,
      left_operand,
      catalog_scope,
      contract_negotiation_scope,
      transfer_process_scope,
      scopes,
      expression,
      description,
      on_create,
    )| {
      event.prevent_default();

      let edc_connector_context = edc_connector_context.clone();
      let identifier = (**identifier).clone();
      let left_operand = (**left_operand).clone();
      let catalog_scope = **catalog_scope;
      let contract_negotiation_scope = **contract_negotiation_scope;
      let transfer_process_scope = **transfer_process_scope;
      let mut scopes = (**scopes).clone();
      let expression = (**expression).clone();
      let description = (**description).clone();
      let on_create = on_create.clone();

      if catalog_scope {
        scopes.push("catalog".to_string());
      }
      if contract_negotiation_scope {
        scopes.push("contract.negotiation".to_string());
      }
      if transfer_process_scope {
        scopes.push("transfer.process".to_string());
      }

      spawn_local(async move {
        let new_common_expression_language_builder = NewCommonExpressionLanguage::builder();

        let new_common_expression_language_builder = new_common_expression_language_builder
          .expression(expression)
          .left_operand(left_operand)
          .description(description)
          .scopes(scopes);

        let new_common_expression_language = if identifier.is_empty() {
          new_common_expression_language_builder.build()
        } else {
          new_common_expression_language_builder
            .id(identifier)
            .build()
        };

        if let Some(client) = edc_connector_context.get_client() {
          if let Err(error) = client
            .common_expression_language(EdcConnectorApiVersion::V5Beta)
            .create(&new_common_expression_language)
            .await
          {
            log::error!("Error creating asset: {}", error);
          } else {
            on_create.emit(());
          }
        }
      })
    },
  );

  let disabled = false;

  let onchange_catalog_scope = use_callback(
    catalog_scope.setter(),
    |checkbox_state, catalog_scope_setter| {
      catalog_scope_setter.set(checkbox_state == CheckboxState::Checked);
    },
  );
  let onchange_contract_negotiation_scope = use_callback(
    contract_negotiation_scope.setter(),
    |checkbox_state, contract_negotiation_setter| {
      contract_negotiation_setter.set(checkbox_state == CheckboxState::Checked)
    },
  );
  let onchange_transfer_process_scope = use_callback(
    transfer_process_scope.setter(),
    |checkbox_state, transfer_process_setter| {
      transfer_process_setter.set(checkbox_state == CheckboxState::Checked)
    },
  );

  html!(
    <Form {onsubmit}>
      <FormGroup label="Identifier" required=true>
        <TextInput required=true value={(*identifier).to_string()} onchange={onchange_identifier} />
      </FormGroup>
      <FormGroup label="Left Operand" required=true>
        <TextInput
          required=true
          value={(*left_operand).to_string()}
          onchange={onchange_left_operand}
        />
      </FormGroup>
      <FormGroup label="Scopes">
        <Stack gutter=true>
          <StackItem>
            <Checkbox label="Catalog" onchange={onchange_catalog_scope} checked={*catalog_scope} />
          </StackItem>
          <StackItem>
            <Checkbox
              label="Contract Negotiation"
              onchange={onchange_contract_negotiation_scope}
              checked={*contract_negotiation_scope}
            />
          </StackItem>
          <StackItem>
            <Checkbox
              label="Transfer Process"
              onchange={onchange_transfer_process_scope}
              checked={*transfer_process_scope}
            />
          </StackItem>
          <StackItem>
            <StringListEdit
              values={(*scopes).clone()}
              onchange={onchange_scopes}
              add_button_label="Add Extra Scope"
            />
          </StackItem>
        </Stack>
      </FormGroup>
      <FormGroupValidated<TextArea>
        label="Expression"
        required=true
        validator={Validator::from(|ctx: ValidationContext<String>| {
            if ctx.initial {
                ValidationResult::default()
            } else if ctx.value.is_empty() {
                ValidationResult::error("Must not be empty")
            } else {
              match cel::Program::compile(&ctx.value) {
                Ok(_) => ValidationResult::new(InputState::Success, ""),
                Err(errors) => ValidationResult::error(errors.to_string())
              }
            }
        })}
      >
        <TextArea required=true value={(*expression).to_string()} onchange={onchange_expression} />
      </FormGroupValidated<TextArea>>
      <FormGroup label="Description">
        <TextArea value={(*description).to_string()} onchange={onchange_description} />
      </FormGroup>
      <ActionGroup>
        <Button
          variant={ButtonVariant::Primary}
          label="Submit"
          r#type={ButtonType::Submit}
          {disabled}
        />
        <Button variant={ButtonVariant::Secondary} label="Reset" r#type={ButtonType::Reset} />
      </ActionGroup>
    </Form>
  )
}
