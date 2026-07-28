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
use crate::contexts::use_edc_connector_context;
use edc_connector_client::EdcConnectorApiVersion;
use edc_connector_client::types::policy::{
  Action, Constraint, NewPolicyDefinition, Obligation, Permission, Policy, PolicyKind, Prohibition,
  Target,
};
use patternfly_yew::prelude::*;
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, strum::Display)]
enum Options {
  Agreement,
  Offer,
  Set,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CreatePolicyProps {
  #[prop_or_default]
  pub on_create: Callback<()>,
}

#[component]
pub fn CreatePolicy(props: &CreatePolicyProps) -> Html {
  let edc_connector_context = use_edc_connector_context();

  let identifier = use_state(String::new);
  let kind = use_state(|| Options::Set);
  let assignee = use_state(String::default);
  let assigner = use_state(String::default);
  let target = use_state(|| (true, String::default()));
  let permissions = use_state(Vec::new);
  let prohibitions = use_state(Vec::new);
  let obligations = use_state(Vec::new);

  let onsubmit = use_callback(
    (
      edc_connector_context,
      identifier.clone(),
      kind.clone(),
      assignee.clone(),
      assigner.clone(),
      target.clone(),
      permissions.clone(),
      prohibitions.clone(),
      obligations.clone(),
      props.on_create.clone(),
    ),
    |event: SubmitEvent,
     (
      edc_connector_context,
      identifier,
      kind,
      assignee,
      assigner,
      target,
      permissions,
      prohibitions,
      obligations,
      on_create,
    )| {
      event.prevent_default();

      let edc_connector_context = edc_connector_context.clone();
      let identifier = (**identifier).clone();
      let kind = **kind;
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

      let on_create = on_create.clone();

      spawn_local(async move {
        let kind = match kind {
          Options::Agreement => PolicyKind::Agreement,
          Options::Offer => PolicyKind::Offer,
          Options::Set => PolicyKind::Set,
        };

        let policy_builder = Policy::builder()
          .kind(kind)
          .permissions(permissions)
          .prohibitions(prohibitions)
          .obligations(obligations);

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
          let _ = client
            .policies(EdcConnectorApiVersion::V4)
            .create(&new_policy)
            .await;
          on_create.emit(());
        }
      })
    },
  );

  let onchange_identifier =
    use_callback(identifier.setter(), move |identifier, identifier_setter| {
      identifier_setter.set(identifier);
    });

  let onselect_kind = use_callback(kind.setter(), move |kind, kind_setter| {
    kind_setter.set(kind);
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

  let (target_is_simple, target_value) = (*target).clone();

  let disabled = false;

  html!(
    <Form {onsubmit}>
      <FormGroup label="Identifier" required=true>
        <TextInput required=true value={(*identifier).to_string()} onchange={onchange_identifier} />
      </FormGroup>
      <FormGroup label="Kind" required=true>
        <SimpleSelect<Options>
          selected={*kind}
          onselect={onselect_kind}
          entries={vec![Options::Agreement, Options::Offer, Options::Set]}
        />
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
