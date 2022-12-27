#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("Just an example")]
  ExampleError,
}
