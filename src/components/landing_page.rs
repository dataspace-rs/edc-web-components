use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew::suspense::use_future_with;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LandingPageProps {
  pub landing_page: String,
}

#[component]
pub fn LandingPage(props: &LandingPageProps) -> Html {
  let backdropper = use_backdrop();

  let onclick = use_callback(
    (backdropper.clone(), props.landing_page.clone()),
    |_, (backdropper, landing_page)| {
      if let Some(backdropper) = backdropper {
        let landing_page = landing_page.clone();

        backdropper.open(Backdrop::new(html!(
          <Bullseye>
            <Modal title="Dataset preview" variant={ModalVariant::Large}>
              <Suspense fallback={html!(<Bullseye><Spinner /></Bullseye>)}>
                <LandingPageInner {landing_page} />
              </Suspense>
            </Modal>
          </Bullseye>
        )))
      }
    },
  );

  html!(<Button variant={ButtonVariant::Link} {onclick}>{ "Preview" }</Button>)
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct LandingPageInnerProps {
  landing_page: String,
}

#[component]
fn LandingPageInner(props: &LandingPageInnerProps) -> HtmlResult {
  let content = use_future_with(props.landing_page.clone(), async move |parameters| {
    let landing_page = (*parameters).clone();

    if let Ok(response) = reqwest::get(&landing_page).await {
      match response
        .headers()
        .get("content-type")
        .map(|header| header.to_str().unwrap().to_string())
        .unwrap_or_default()
        .as_str()
      {
        "text/html" => {
          html!(
            <iframe
              title="Dataset Preview"
              src={landing_page}
              id="viewer"
              width="100%"
              height="100%"
              style="height: 80vh"
              frameborder="0"
              allow="fullscreen"
            />
          )
        }
        header if header.starts_with("video/") => {
          let header = header.to_string();

          html!(
            <video controls=true width="100%">
              <source src="/shared-assets/videos/flower.webm" type={header} />
            </video>
          )
        }
        _ => html!(
          <iframe
            class="viewer"
            data-src="{landing_page}"
            height="100%"
            width="100%"
            allow="fullscreen"
          />
        ),
      }
    } else {
      html!(
        <iframe
          title="Dataset Preview"
          src={landing_page}
          id="viewer"
          width="100%"
          height="100%"
          style="height: 80vh"
          frameborder="0"
          allow="fullscreen"
        />
      )
    }
  })?;

  let content = (*content).clone();

  Ok(content)
}
