use crate::contexts::use_edc_connector_context;
use edc_connector_client::EdcConnectorApiVersion;
use edc_connector_client::types::properties::ToValue;
use edc_connector_client::types::{asset::NewAsset, data_address::DataAddress};
use patternfly_yew::prelude::*;
use std::collections::HashMap;
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CreateAssetProps {
  #[prop_or_default]
  pub on_create: Callback<()>,
}

#[component]
pub fn CreateAsset(props: &CreateAssetProps) -> Html {
  let edc_connector_context = use_edc_connector_context();

  let identifier = use_state(|| "".to_string());
  let name = use_state(|| "".to_string());
  let description = use_state(|| "".to_string());
  let thumbnail_url = use_state(|| "".to_string());
  let base_url = use_state(|| "".to_string());
  let company_name = use_state(|| "".to_string());
  let company_logo_url = use_state(|| "".to_string());
  let content_type = use_state(|| "application/json".to_string());
  let proxy_path = use_state(|| false);
  let proxy_query_params = use_state(|| false);
  let proxy_method = use_state(|| false);
  let proxy_body = use_state(|| false);
  let headers = use_state(HashMap::<String, String>::new);

  let onsubmit = use_callback(
    (
      edc_connector_context.clone(),
      identifier.clone(),
      name.clone(),
      base_url.clone(),
      content_type.clone(),
      proxy_path.clone(),
      proxy_query_params.clone(),
      proxy_method.clone(),
      proxy_body.clone(),
      headers.clone(),
      props.on_create.clone(),
    ),
    |event: SubmitEvent,
     (
      edc_connector_context,
      identifier,
      name,
      base_url,
      content_type,
      proxy_path,
      proxy_query_params,
      proxy_method,
      proxy_body,
      headers,
      on_create,
    )| {
      event.prevent_default();

      let identifier = (**identifier).clone();
      let name = (**name).clone();
      let base_url = (**base_url).clone();
      let content_type = (**content_type).clone();
      let proxy_path = **proxy_path;
      let proxy_query_params = **proxy_query_params;
      let proxy_method = **proxy_method;
      let proxy_body = **proxy_body;
      let headers = (**headers).clone();
      let edc_connector_context = edc_connector_context.clone();
      let on_create = on_create.clone();

      spawn_local(async move {
        let mut data_address_builder = DataAddress::builder()
          .kind("HttpData")
          .property("baseUrl", base_url)
          .property("proxyPath", if proxy_path { "true" } else { "false" })
          .property(
            "proxyQueryParams",
            if proxy_query_params { "true" } else { "false" },
          )
          .property("proxyMethod", if proxy_method { "true" } else { "false" })
          .property("proxyBody", if proxy_body { "true" } else { "false" });

        struct DcatService {
          title: String,
        }
        impl ToValue for DcatService {
          fn into_value(self) -> serde_json::Value {
            serde_json::json!({
              "@type": "http://www.w3.org/ns/dcat#DataService",
              "dcterms:title": self.title
            })
          }
        }

        for (key, value) in &headers {
          data_address_builder = data_address_builder.property(&format!("header:{key}"), value);
        }

        let data_address = data_address_builder.build().unwrap();

        let new_asset = NewAsset::builder()
          .id(&identifier)
          .data_address(data_address)
          .property("name", name)
          .property("contenttype", content_type)
          .property(
            "http://www.w3.org/ns/dcat#service",
            DcatService {
              title: "test".to_string(),
            },
          )
          .property(
            "http://purl.org/dc/terms/title",
            DcatService {
              title: "test".to_string(),
            },
          )
          .build();

        if let Some(client) = edc_connector_context.get_client() {
          if let Err(error) = client
            .assets(EdcConnectorApiVersion::V4)
            .create(&new_asset)
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

  let onchange_identifier = use_callback(identifier.setter(), |value, identifier_setter| {
    identifier_setter.set(value);
  });

  let onchange_name = use_callback(name.setter(), |value, name_setter| {
    name_setter.set(value);
  });

  let onchange_description = use_callback(description.setter(), |value, description_setter| {
    description_setter.set(value);
  });

  let onchange_thumbnail_url =
    use_callback(thumbnail_url.setter(), |value, thumbnail_url_setter| {
      thumbnail_url_setter.set(value);
    });

  let onchange_company_name = use_callback(company_name.setter(), |value, company_name_setter| {
    company_name_setter.set(value);
  });

  let onchange_company_logo_url = use_callback(
    company_logo_url.setter(),
    |value, company_logo_url_setter| {
      company_logo_url_setter.set(value);
    },
  );

  let onchange_base_url = use_callback(base_url.setter(), |value, base_url_setter| {
    base_url_setter.set(value);
  });

  let onchange_content_type = use_callback(content_type.setter(), |value, content_type_setter| {
    content_type_setter.set(value);
  });

  let onchange_proxy_path = use_callback(proxy_path.setter(), |value, proxy_path_setter| {
    proxy_path_setter.set(value);
  });

  let onchange_proxy_query_params = use_callback(
    proxy_query_params.setter(),
    |value, proxy_query_params_setter| {
      proxy_query_params_setter.set(value);
    },
  );

  let onchange_proxy_method = use_callback(proxy_method.setter(), |value, proxy_method_setter| {
    proxy_method_setter.set(value);
  });

  let onchange_proxy_body = use_callback(proxy_body.setter(), |value, proxy_body_setter| {
    proxy_body_setter.set(value);
  });

  let disabled = (*name).is_empty() || (*base_url).is_empty();

  html!(
    <Form {onsubmit}>
      <FormGroup label="Identifier">
        <TextInput required=true value={(*identifier).to_string()} onchange={onchange_identifier} />
      </FormGroup>
      <FormGroup label="Title" required=true>
        <TextInput required=true value={(*name).to_string()} onchange={onchange_name} />
      </FormGroup>
      <FormGroup label="Description">
        <TextArea required=true value={(*description).to_string()} onchange={onchange_description} />
      </FormGroup>
      <FormGroup label="Thumbnail URL">
        <TextInput
          required=true
          value={(*thumbnail_url).to_string()}
          onchange={onchange_thumbnail_url}
        />
      </FormGroup>
      <FormGroup label="Company Name">
        <TextArea
          required=true
          value={(*company_name).to_string()}
          onchange={onchange_company_name}
        />
      </FormGroup>
      <FormGroup label="Company Logo URL">
        <TextInput
          required=true
          value={(*company_logo_url).to_string()}
          onchange={onchange_company_logo_url}
        />
      </FormGroup>
      <FormGroup label="Base URL" required=true>
        <TextInput
          required=true
          value={(*base_url).to_string()}
          onchange={onchange_base_url}
          r#type={TextInputType::Url}
        />
      </FormGroup>
      <FormGroup label="Content Type">
        <TextInput value={(*content_type).to_string()} onchange={onchange_content_type} />
      </FormGroup>
      <FormGroup label="Proxy Path">
        <Switch checked={*proxy_path} onchange={onchange_proxy_path} />
      </FormGroup>
      <FormGroup label="Proxy Query Parameters">
        <Switch checked={*proxy_query_params} onchange={onchange_proxy_query_params} />
      </FormGroup>
      <FormGroup label="Proxy Method">
        <Switch checked={*proxy_method} onchange={onchange_proxy_method} />
      </FormGroup>
      <FormGroup label="Proxy Body">
        <Switch checked={*proxy_body} onchange={onchange_proxy_body} />
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
