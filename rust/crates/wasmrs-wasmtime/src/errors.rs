/// This crate's Error type
#[derive(thiserror::Error, Debug)]
pub enum Error {
  /// WASMTime initialization failed
  #[error("Initialization failed: {0}")]
  InitializationFailed(Box<dyn std::error::Error + Send + Sync>),

  /// Error originating from [wasi_common]
  #[error("{0}")]
  WasiError(#[from] wasi_common::Error),

  /// Thrown if the guest's send function is not exported.
  #[error("Guest init function not exported by wasm module.")]
  GuestInit,

  /// Thrown if the guest's send function is not exported.
  #[error("Guest send function not exported by wasm module.")]
  GuestSend,

  /// Thrown if the host has a problem reading the guest's memory.
  #[error("Could not read guest memory")]
  GuestMemory,
}

impl From<Error> for wasmrs::Error {
  fn from(e: Error) -> Self {
    match e {
      Error::InitializationFailed(_) => wasmrs::Error::RSocket(wasmrs::ErrorCode::ConnectionError.into()),
      Error::WasiError(_) => wasmrs::Error::RSocket(wasmrs::ErrorCode::ConnectionError.into()),
      Error::GuestInit => wasmrs::Error::RSocket(wasmrs::ErrorCode::ApplicationError.into()),
      Error::GuestSend => wasmrs::Error::RSocket(wasmrs::ErrorCode::ApplicationError.into()),
      Error::GuestMemory => wasmrs::Error::RSocket(wasmrs::ErrorCode::Canceled.into()),
    }
  }
}
