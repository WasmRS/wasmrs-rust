use std::sync::Arc;

use bytes::Bytes;
use parking_lot::Mutex;
use wasmrs::{Frame, OperationList, WasmSocket};

type Result<T> = std::result::Result<T, crate::errors::Error>;

#[derive(Clone)]
#[allow(missing_debug_implementations)]
#[allow(missing_docs)]
pub struct SharedContext(Arc<Mutex<dyn ProviderCallContext + Send + Sync + 'static>>);

impl SharedContext {
  /// Create a new shared context with the passed [ProviderCallContext]
  pub fn new(context: impl ProviderCallContext + Send + Sync + 'static) -> Self {
    Self(Arc::new(Mutex::new(context)))
  }

  pub(crate) fn init(&self) -> Result<()> {
    self.0.lock().init()
  }

  pub(crate) fn write_frame(&self, frame: Frame) -> Result<()> {
    let result = self.0.lock().write_frame(frame);

    if let Err(e) = &result {
      error!("send request_response failed: {}", e);
    }
    Ok(result?)
  }

  pub(crate) fn get_import(&self, namespace: &str, operation: &str) -> Result<u32> {
    Ok(self.0.lock().get_import(namespace, operation)?)
  }

  pub(crate) fn get_export(&self, namespace: &str, operation: &str) -> Result<u32> {
    Ok(self.0.lock().get_export(namespace, operation)?)
  }

  pub(crate) fn get_operation_list(&self) -> OperationList {
    self.0.lock().get_operation_list()
  }
}

/// All engine providers must implement the [EngineProvider] trait.
pub trait EngineProvider {
  /// Initializes the [EngineProvider]
  fn init(&mut self) -> Result<()> {
    Ok(())
  }

  /// Called to create a new [SharedContext].
  fn new_context(&self, state: Arc<WasmSocket>) -> Result<SharedContext>;
}

/// The trait implemented by a context for a call or set of calls.
pub trait ProviderCallContext: wasmrs::ModuleHost {
  /// Initialize the call context.
  fn init(&mut self) -> Result<()>;
}

/// The trait that a host needs to implement to satisfy wasmrs protocol imports and to query data about the loaded module.
pub trait CallbackProvider {
  /// The callback for `__wasmrs_send`
  fn do_host_send(&self, frame_bytes: Bytes) -> Result<()>;
  #[allow(missing_docs)]
  fn do_console_log(&self, msg: &str);
  /// Query the operation list for the module.
  fn do_op_list(&self, bytes: Bytes) -> Result<()>;
  /// The callback for `__wasmrs_init`
  fn do_host_init(&self, guest_buff_ptr: u32, host_buff_ptr: u32) -> Result<()>;
  /// Find an import id by namespace and operation.
  fn get_import(&self, namespace: &str, operation: &str) -> Result<u32>;
  /// Find an export id by namespace and operation.
  fn get_export(&self, namespace: &str, operation: &str) -> Result<u32>;
}
