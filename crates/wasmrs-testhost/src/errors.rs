/// This crate's Error type
#[derive(thiserror::Error, Debug)]
pub enum Error {
  /// WASMTime initialization failed
  #[error("Initialization failed: {0}")]
  InitializationFailed(Box<dyn std::error::Error + Send + Sync>),

  /// WASMTime initialization failed
  #[error("Initialization failed: {0}")]
  Initialization(anyhow::Error),

  /// WASMTime Linker initialization failed
  #[error("Linker initialization failed: {0}")]
  Linker(anyhow::Error),

  /// Setting up linked functions failed.
  #[error("Could not create WebAssembly function: {0}")]
  Func(anyhow::Error),

  /// WASMTime module instantiation failed
  #[error("Could not instantiate new WASM Module: {0}")]
  Module(anyhow::Error),

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
    let code = match e {
      Error::InitializationFailed(_) => wasmrs::ErrorCode::ConnectionError,
      Error::Initialization(_) => wasmrs::ErrorCode::ConnectionError,
      Error::Func(_) => wasmrs::ErrorCode::ConnectionError,
      Error::Linker(_) => wasmrs::ErrorCode::ConnectionError,
      Error::Module(_) => wasmrs::ErrorCode::ConnectionError,
      Error::WasiError(_) => wasmrs::ErrorCode::ConnectionError,
      Error::GuestInit => wasmrs::ErrorCode::ApplicationError,
      Error::GuestSend => wasmrs::ErrorCode::ApplicationError,
      Error::GuestMemory => wasmrs::ErrorCode::Canceled,
    };
    wasmrs::Error::RSocket(code.into())
  }
}
