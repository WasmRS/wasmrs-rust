#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error(transparent)]
  PayloadError(#[from] wasmrs_guest::PayloadError),
  #[error(transparent)]
  Protocol(#[from] wasmrs_guest::Error),

  #[error("An example of a custom error variant")]
  ExampleError,
}
