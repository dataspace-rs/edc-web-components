use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct CounterParty {
  pub did: String,
  pub name: String,
  pub logo: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct CounterPartiesState {
  counter_parties: Vec<CounterParty>,
}

impl CounterPartiesState {
  pub fn counter_parties(&self) -> &[CounterParty] {
    &self.counter_parties
  }
}

#[derive(Properties, PartialEq)]
pub struct CounterPartiesContextProviderProps {
  #[prop_or_default]
  pub children: Html,
  pub counter_parties: Vec<CounterParty>,
}

#[component]
pub fn CounterPartiesContextProvider(props: &CounterPartiesContextProviderProps) -> Html {
  let counter_parties_context = use_state(move || CounterPartiesState {
    counter_parties: props.counter_parties.clone(),
  });

  use_effect_with(
    (
      props.counter_parties.clone(),
      counter_parties_context.setter(),
    ),
    |(counter_parties, counter_parties_context_setter)| {
      counter_parties_context_setter.set(CounterPartiesState {
        counter_parties: counter_parties.clone(),
      });
    },
  );

  html! {
    <ContextProvider<CounterPartiesState> context={(*counter_parties_context).clone()}>
      { props.children.clone() }
    </ContextProvider<CounterPartiesState>>
  }
}

#[hook]
pub fn use_counter_parties_context() -> Option<CounterPartiesState> {
  use_context::<CounterPartiesState>()
}
