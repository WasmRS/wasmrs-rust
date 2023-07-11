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

  /// WASMTime module instantiation failed
  #[error("Could not find module {0} in module cache")]
  NotFound(String),

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

  /// Thrown if the builder wasn't provide a module to instantiate with.
  #[error("Must provide a module to the builder")]
  NoModule,

  /// Thrown if the builder was provided too many module options.
  #[error("Must provide either module bytes with ID to cache or a cached ID, not both")]
  AmbiguousModule,
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
      Error::NotFound(_) => wasmrs::ErrorCode::ApplicationError,
      Error::NoModule => wasmrs::ErrorCode::ApplicationError,
      Error::AmbiguousModule => wasmrs::ErrorCode::ApplicationError,
    };
    wasmrs::Error::RSocket(code.into())
  }
}
