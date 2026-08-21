use crate::models::DataspaceDataset;
use patternfly_yew::prelude::*;
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct DatasetCardProps {
  pub dataset: DataspaceDataset,
  #[prop_or_default]
  pub on_offer_click: Option<Callback<()>>,
  #[prop_or(AttrValue::Static("Select"))]
  pub button_label: AttrValue,
}

#[styled_component]
pub fn DatasetCard(props: &DatasetCardProps) -> Html {
  let card_class = css!(
    width: 250px;
  );

  let thumbnails_class = css!(
    height: 180px;
    object-fit: cover;
  );

  let provider_logo_class = css!(
    width: 50px;
    height: 50px;
    object-fit: contain;
  );

  let version_class = css!(
    position: absolute;
    top: 5px;
    right: 5px;
  );

  let keyword_class = css!(
    .pf-v6-l-split {
      overflow: hidden;
    }
  );

  let description_class = css!(
    display: block;
    display: -webkit-box;
    -webkit-line-clamp: 5;
    -webkit-box-orient: vertical;
    text-overflow: ellipsis;
    overflow: hidden;
  );

  let offer_button_class = css!(
    text-align: right;
  );

  let offer_button = if let Some(offer) = props.on_offer_click.clone() {
    html!(
      <Button
        variant={ButtonVariant::Primary}
        icon={Icon::AngleRight}
        onclick={offer.reform(|_| ())}
      >
        { props.button_label.clone() }
      </Button>
    )
  } else {
    html!()
  };

  let title = props.dataset.title.to_string();

  let version = props
    .dataset
    .version
    .clone()
    .map(|version| version.to_string())
    .unwrap_or_default();

  let comment = props.dataset.comment.clone().unwrap_or_default();

  let thumbnail = props
    .dataset
    .thumbnail
    .clone()
    .and_then(|thumbnail| thumbnail.resource)
    .map(|thumbnail| html! { <img src={thumbnail} class={thumbnails_class.clone()} /> });

  let provider_logo = props
    .dataset
    .creator
    .clone()
    .and_then(|creator| creator.thumbnail)
    .and_then(|thumbnail| thumbnail.resource)
    .map(|thumbnail| html! { <img src={thumbnail} class={provider_logo_class.clone()} /> });

  let keywords = props.dataset.keywords.iter().map(|keyword| {
    html_nested! {
      <SplitItem>
        <Label label={keyword.clone()} />
      </SplitItem>
    }
  });

  html!(
    <Card full_height=true class={card_class.clone()}>
      { thumbnail }
      <div class={version_class.clone()}>
        <Badge read=true>{ "v" }{ version }</Badge>
      </div>
      <CardTitle>
        <div>{ provider_logo }</div>
        <Truncate content={title} />
      </CardTitle>
      <CardBody>
        <Stack gutter=true>
          <StackItem fill=true>
            <div class={description_class.clone()}>{ comment }</div>
          </StackItem>
          <StackItem>
            <div class={offer_button_class.clone()}>{ offer_button.clone() }</div>
          </StackItem>
        </Stack>
      </CardBody>
      <CardFooter>
        <slot class={keyword_class.clone()}>
          <Split gutter=true>{ for keywords }</Split>
        </slot>
      </CardFooter>
    </Card>
  )
}
