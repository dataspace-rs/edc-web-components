#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display)]
pub enum ConsumerProvider {
  Consumer,
  Provider,
}
