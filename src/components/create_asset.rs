use crate::components::{DatasetCard, StringListEdit};
use crate::contexts::use_edc_connector_context;
use crate::models::{Creator, DataspaceDataset, Thumbnail};
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
  pub company_name: Option<String>,
  #[prop_or_default]
  pub company_logo_url: Option<String>,
  #[prop_or_default]
  pub on_create: Callback<()>,
}

#[component]
pub fn CreateAsset(props: &CreateAssetProps) -> Html {
  let edc_connector_context = use_edc_connector_context();

  let name = use_state(String::new);
  let version = use_state(String::new);
  let description = use_state(String::new);
  let thumbnail_url = use_state(String::new);
  let keywords = use_state(Vec::<String>::new);
  let base_url = use_state(String::new);
  let company_name = use_state(|| props.company_name.clone().unwrap_or_default());
  let company_logo_url = use_state(|| props.company_logo_url.clone().unwrap_or_default());
  let content_type = use_state(|| "application/json".to_string());
  let proxy_path = use_state(|| false);
  let proxy_query_params = use_state(|| false);
  let proxy_method = use_state(|| false);
  let proxy_body = use_state(|| false);
  let headers = use_state(HashMap::<String, String>::new);

  let onsubmit = use_callback(
    (
      edc_connector_context.clone(),
      (
        name.clone(),
        version.clone(),
        description.clone(),
        thumbnail_url.clone(),
        keywords.clone(),
      ),
      base_url.clone(),
      (company_name.clone(), company_logo_url.clone()),
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
      (name, version, description, thumbnail_url, keywords),
      base_url,
      (company_name, company_logo_url),
      content_type,
      proxy_path,
      proxy_query_params,
      proxy_method,
      proxy_body,
      headers,
      on_create,
    )| {
      event.prevent_default();

      let name = (**name).clone();
      let version = (**version).clone();
      let description = (**description).clone();
      let thumbnail_url = (**thumbnail_url).clone();
      let keywords = (**keywords).clone();
      let base_url = (**base_url).clone();
      let company_name = (**company_name).clone();
      let company_logo_url = (**company_logo_url).clone();
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

        struct Creator {
          name: String,
          logo_url: String,
        }

        impl ToValue for Creator {
          fn into_value(self) -> serde_json::Value {
            serde_json::json!({
              "@type": [
                "http://xmlns.com/foaf/0.1/Organization",
                "http://www.w3.org/ns/prov#Agent"
              ],
              "http://xmlns.com/foaf/0.1/name": self.name,
              "http://xmlns.com/foaf/0.1/thumbnail": {
                "rdf:resource": self.logo_url,
              },
            })
          }
        }

        struct Thumbnail {
          resource: String,
        }

        impl ToValue for Thumbnail {
          fn into_value(self) -> serde_json::Value {
            serde_json::json!({
              "rdf:resource": self.resource,
            })
          }
        }

        for (key, value) in &headers {
          data_address_builder = data_address_builder.property(&format!("header:{key}"), value);
        }

        let data_address = data_address_builder.build().unwrap();

        let new_asset_builder = NewAsset::builder()
          // .id(&identifier)
          .data_address(data_address)
          .property("name", name.clone())
          .property("http://www.w3.org/ns/dcat#version", version)
          .property("contenttype", content_type)
          .property(
            "http://purl.org/dc/terms/creator",
            Creator {
              name: company_name,
              logo_url: company_logo_url,
            },
          );

        let new_asset_builder = if !name.is_empty() {
          new_asset_builder.property("http://purl.org/dc/terms/title", name)
        } else {
          new_asset_builder
        };

        let new_asset_builder = if !description.is_empty() {
          new_asset_builder.property("http://www.w3.org/2000/01/rdf-schema#comment", description)
        } else {
          new_asset_builder
        };

        let new_asset_builder = if !thumbnail_url.is_empty() {
          new_asset_builder.property(
            "http://xmlns.com/foaf/0.1/thumbnail",
            Thumbnail {
              resource: thumbnail_url,
            },
          )
        } else {
          new_asset_builder
        };

        let new_asset_builder = if !keywords.is_empty() {
          new_asset_builder.property("http://www.w3.org/ns/dcat#keyword", keywords)
        } else {
          new_asset_builder
        };

        let new_asset = new_asset_builder.build();

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

  let onchange_name = use_callback(name.setter(), |value, name_setter| {
    name_setter.set(value);
  });

  let onchange_version = use_callback(version.setter(), |value, version_setter| {
    version_setter.set(value);
  });

  let onchange_description = use_callback(description.setter(), |value, description_setter| {
    description_setter.set(value);
  });

  let onchange_thumbnail_url =
    use_callback(thumbnail_url.setter(), |value, thumbnail_url_setter| {
      thumbnail_url_setter.set(value);
    });

  let onchange_keywords = use_callback(keywords.setter(), |keywords, keywords_setter| {
    keywords_setter.set(keywords);
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

  let dataset = DataspaceDataset {
    id: "".to_string(),
    title: (*name).clone(),
    version: semver::Version::parse(&version).ok(),
    comment: if (*description).is_empty() {
      None
    } else {
      Some((*description).clone())
    },
    thumbnail: if (*thumbnail_url).is_empty() {
      None
    } else {
      Some(Thumbnail {
        resource: Some((*thumbnail_url).clone()),
      })
    },
    creator: if (*company_logo_url).is_empty() {
      None
    } else {
      Some(Creator {
        name: None,
        thumbnail: Some(Thumbnail {
          resource: Some((*company_logo_url).clone()),
        }),
      })
    },
    keywords: (*keywords).clone(),
    policies: vec![],
    dcterm_types: vec![],
  };

  html!(
    <Form {onsubmit}>
      <Card>
        <CardHeader>
          <Title level={Level::H2} size={Size::XXXLarge}>{ "Provider Information" }</Title>
        </CardHeader>
        <CardBody>
          <FormGroup label="Company Name">
            <TextInput
              required=true
              value={(*company_name).to_string()}
              onchange={onchange_company_name}
              readonly={props.company_name.is_some()}
            />
          </FormGroup>
          <FormGroup label="Company Logo URL">
            <TextInput
              required=true
              value={(*company_logo_url).to_string()}
              onchange={onchange_company_logo_url}
              readonly={props.company_logo_url.is_some()}
            />
          </FormGroup>
        </CardBody>
      </Card>
      <Card>
        <CardHeader>
          <Title level={Level::H2} size={Size::XXXLarge}>{ "Dataset Description" }</Title>
        </CardHeader>
        <CardBody>
          <Flex>
            <FlexItem
              modifiers={[FlexModifier::Flex1.all(), FlexModifier::Align(Alignment::Start).all()]}
            >
              <FormGroup label="Title" required=true>
                <TextInput required=true value={(*name).to_string()} onchange={onchange_name} />
              </FormGroup>
              <FormGroupValidated<TextInput>
                label="Version"
                required=true
                validator={Validator::from(|ctx: ValidationContext<String>| {
                  if ctx.initial {
                    ValidationResult::default()
                  } else if ctx.value.is_empty() {
                    ValidationResult::error("Must not be empty")
                  } else {
                    if semver::Version::parse(&ctx.value).is_ok() {
                      ValidationResult::ok()
                    } else {
                      ValidationResult::error("Not a semantic versioning format (MAJOR.MINOR.PATCH)")
                    }
                  }
               })}
              >
                <TextInput
                  required=true
                  value={(*version).to_string()}
                  onchange={onchange_version}
                />
              </FormGroupValidated<TextInput>>
              <FormGroup label="Description">
                <TextArea
                  required=true
                  value={(*description).to_string()}
                  onchange={onchange_description}
                />
              </FormGroup>
              <FormGroup label="Thumbnail URL">
                <TextInput
                  required=true
                  value={(*thumbnail_url).to_string()}
                  onchange={onchange_thumbnail_url}
                />
              </FormGroup>
              <FormGroup label="Keywords">
                <StringListEdit values={(*keywords).clone()} onchange={onchange_keywords} />
              </FormGroup>
            </FlexItem>
            <FlexItem modifiers={[FlexModifier::Align(Alignment::Start).all()]}>
              <DatasetCard {dataset} />
            </FlexItem>
          </Flex>
        </CardBody>
      </Card>
      <Card>
        <CardHeader>
          <Title level={Level::H2} size={Size::XXXLarge}>{ "Dataset Source" }</Title>
        </CardHeader>
        <CardBody>
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
        </CardBody>
      </Card>
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
