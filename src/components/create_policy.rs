mod atomic_constraint_edit;
mod list_of_rules;
mod operator_selector;
mod rule;

pub use self::atomic_constraint_edit::AtomicConstraintEdit;
pub use self::list_of_rules::ListOfRules;
pub use self::operator_selector::OperatorSelector;
pub use self::rule::Rule;
use crate::components::create_policy::atomic_constraint_edit::ConstraintMode;
use crate::components::simple_or_id_field::SimpleOrIdField;
use crate::components::{ExtensiblePropertiesEdit, StringListEdit};
use crate::contexts::use_edc_connector_context;
use edc_connector_client::types::policy::{
  Action, Constraint, NewPolicyDefinition, Obligation, Permission, Policy, PolicyKind, Prohibition,
  Target,
};
use edc_connector_client::{EdcConnectorApiVersion, Error, ManagementApiErrorDetailKind};
use patternfly_yew::prelude::*;
use std::collections::HashMap;
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CreatePolicyProps {
  #[prop_or_default]
  pub on_create: Callback<()>,
}

#[component]
pub fn CreatePolicy(props: &CreatePolicyProps) -> Html {
  let edc_connector_context = use_edc_connector_context();

  let identifier = use_state(String::new);
  let assignee = use_state(String::default);
  let assigner = use_state(String::default);
  let target = use_state(|| (true, String::default()));
  let permissions = use_state(Vec::new);
  let prohibitions = use_state(Vec::new);
  let obligations = use_state(Vec::new);
  let profiles = use_state(Vec::new);
  let extensible_properties = use_state(HashMap::new);

  let creation_errors = use_state(|| None);

  let onsubmit = use_callback(
    (
      edc_connector_context,
      identifier.clone(),
      assignee.clone(),
      assigner.clone(),
      target.clone(),
      permissions.clone(),
      prohibitions.clone(),
      obligations.clone(),
      profiles.clone(),
      extensible_properties.clone(),
      props.on_create.clone(),
      creation_errors.setter(),
    ),
    |event: SubmitEvent,
     (
      edc_connector_context,
      identifier,
      assignee,
      assigner,
      target,
      permissions,
      prohibitions,
      obligations,
      profiles,
      extensible_properties,
      on_create,
      creation_errors_setter,
    )| {
      event.prevent_default();

      let edc_connector_context = edc_connector_context.clone();
      let identifier = (**identifier).clone();
      let profiles = (**profiles).clone();
      let extensible_properties = (**extensible_properties).clone();
      let assignee = (**assignee).clone();
      let assigner = (**assigner).clone();
      let (is_simple_target, target) = (**target).clone();

      let permissions = (**permissions)
        .iter()
        .map(
          |(action, constraints): &(Action, Vec<(ConstraintMode, Constraint)>)| {
            Permission::builder()
              .action(action.clone())
              .constraints(
                constraints
                  .iter()
                  .map(|(_, constraint)| constraint.clone())
                  .collect(),
              )
              .build()
          },
        )
        .collect();

      let prohibitions = (**prohibitions)
        .iter()
        .map(
          |(action, constraints): &(Action, Vec<(ConstraintMode, Constraint)>)| {
            Prohibition::builder()
              .action(action.clone())
              .constraints(
                constraints
                  .iter()
                  .map(|(_, constraint)| constraint.clone())
                  .collect(),
              )
              .build()
          },
        )
        .collect();

      let obligations = (**obligations)
        .iter()
        .map(
          |(action, constraints): &(Action, Vec<(ConstraintMode, Constraint)>)| {
            Obligation::builder()
              .action(action.clone())
              .constraints(
                constraints
                  .iter()
                  .map(|(_, constraint)| constraint.clone())
                  .collect(),
              )
              .build()
          },
        )
        .collect();

      creation_errors_setter.set(None);

      let on_create = on_create.clone();
      let creation_errors_setter = creation_errors_setter.clone();

      spawn_local(async move {
        let kind = PolicyKind::Set;

        let policy_builder = Policy::builder()
          .kind(kind)
          .permissions(permissions)
          .prohibitions(prohibitions)
          .obligations(obligations)
          .profiles(profiles)
          .extensible_properties(extensible_properties);

        let policy_builder = if !assignee.is_empty() {
          policy_builder.maybe_assignee(Some(assignee))
        } else {
          policy_builder.maybe_assignee(None::<String>)
        };

        let policy_builder = if !assigner.is_empty() {
          policy_builder.maybe_assigner(Some(assigner))
        } else {
          policy_builder.maybe_assigner(None::<String>)
        };

        let policy_builder = if !target.is_empty() {
          if is_simple_target {
            policy_builder.target(Target::Simple(target))
          } else {
            policy_builder.target(Target::Id { id: target })
          }
        } else {
          policy_builder.maybe_target(None::<Target>)
        };

        let policy = policy_builder.build();

        let new_policy = NewPolicyDefinition::builder()
          .id(&identifier)
          .policy(policy)
          .build();

        if let Some(client) = edc_connector_context.get_client() {
          if let Err(error) = client
            .policies(EdcConnectorApiVersion::V4)
            .create(&new_policy)
            .await
          {
            match error {
              Error::ManagementApi(management_api_error) => {
                let error_message = match management_api_error.error_detail {
                  ManagementApiErrorDetailKind::Raw(error) => html!(<div>{ error }</div>),
                  ManagementApiErrorDetailKind::Parsed(error_list) => error_list
                    .into_iter()
                    .map(|error| html!(<div>{ error.message }</div>))
                    .collect::<Html>(),
                };

                creation_errors_setter.set(Some(error_message));
              }
              _ => {
                let error_message = format!("{error:?}");
                creation_errors_setter.set(Some(html!(<div>{ error_message }</div>)));
              }
            }
          } else {
            on_create.emit(());
          }
        }
      })
    },
  );

  let onchange_identifier =
    use_callback(identifier.setter(), move |identifier, identifier_setter| {
      identifier_setter.set(identifier);
    });

  let onchange_assignee = use_callback(
    assignee.setter(),
    move |assignee: String, assignee_setter| {
      assignee_setter.set(assignee);
    },
  );

  let onchange_assigner = use_callback(
    assigner.setter(),
    move |assigner: String, assigner_setter| {
      assigner_setter.set(assigner);
    },
  );

  let onchange_target = use_callback(
    target.setter(),
    move |target: (bool, String), target_setter| {
      target_setter.set(target);
    },
  );

  let onchange_permissions = use_callback(
    permissions.setter(),
    move |permissions, permissions_setter| {
      permissions_setter.set(permissions);
    },
  );

  let onchange_prohibitions = use_callback(
    prohibitions.setter(),
    move |prohibitions, prohibitions_setter| {
      prohibitions_setter.set(prohibitions);
    },
  );

  let onchange_obligations = use_callback(
    obligations.setter(),
    move |obligations, obligations_setter| {
      obligations_setter.set(obligations);
    },
  );

  let onchange_profiles = use_callback(profiles.setter(), move |profiles, profiles_setter| {
    profiles_setter.set(profiles);
  });

  let onchange_extensible_properties = use_callback(
    extensible_properties.setter(),
    move |extensible_properties, extensible_properties_setter| {
      extensible_properties_setter.set(extensible_properties);
    },
  );

  let (target_is_simple, target_value) = (*target).clone();

  let disabled = false;

  let errors = if let Some(error_mesage) = (*creation_errors).clone() {
    html!(
      <Alert title="Unable to create the policy" r#type={AlertType::Danger}>{ error_mesage }</Alert>
    )
  } else {
    html!()
  };

  html!(
    <Form {onsubmit}>
      { errors }
      <FormGroup label="Identifier" required=true>
        <TextInput required=true value={(*identifier).to_string()} onchange={onchange_identifier} />
      </FormGroup>
      <FormGroup label="Permissions">
        <ListOfRules list={(*permissions).clone()} onchange={onchange_permissions} />
      </FormGroup>
      <FormGroup label="Prohibitions">
        <ListOfRules list={(*prohibitions).clone()} onchange={onchange_prohibitions} />
      </FormGroup>
      <FormGroup label="Obligations">
        <ListOfRules list={(*obligations).clone()} onchange={onchange_obligations} />
      </FormGroup>
      <FormGroup label="Profiles">
        <StringListEdit values={(*profiles).clone()} onchange={onchange_profiles} />
      </FormGroup>
      <FormGroup label="Extensible Properties">
        <ExtensiblePropertiesEdit
          values={(*extensible_properties).clone()}
          onchange={onchange_extensible_properties}
        />
        { format!("{:#?}", (*extensible_properties).clone()) }
      </FormGroup>
      <FormGroup label="Assignee">
        <TextInput value={(*assignee).clone()} onchange={onchange_assignee} />
      </FormGroup>
      <FormGroup label="Assigner">
        <TextInput value={(*assigner).clone()} onchange={onchange_assigner} />
      </FormGroup>
      <FormGroup label="Target">
        <SimpleOrIdField
          onchange={onchange_target}
          is_simple={target_is_simple}
          value={target_value}
        />
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
