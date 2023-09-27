use std::sync::Arc;

use bytes::Bytes;
use wasmrs::{Frame, OperationList, WasmSocket};

use crate::host::HostServer;

type Result<T> = std::result::Result<T, crate::errors::Error>;

#[derive(Clone)]
#[allow(missing_debug_implementations)]
#[allow(missing_docs)]
pub struct SharedContext(Arc<dyn ProviderCallContext + Send + Sync + 'static>);

impl SharedContext {
  /// Create a new shared context with the passed [ProviderCallContext]
  pub fn new(context: impl ProviderCallContext + Send + Sync + 'static) -> Self {
    Self(Arc::new(context))
  }

  pub(crate) async fn init(&self, host_buffer_size: u32, guest_buffer_size: u32) -> Result<()> {
    self.0.init(host_buffer_size, guest_buffer_size).await
  }

  pub(crate) async fn write_frame(&self, frame: Frame) -> Result<()> {
    let id = frame.stream_id();
    let result = self.0.write_frame(frame).await;

    if let Err(e) = &result {
      error!("failed to write frame for stream ID {}: {}", id, e);
      self.0.on_error(id).await?;
    }

    Ok(result?)
  }

  pub(crate) fn get_import(&self, namespace: &str, operation: &str) -> Option<u32> {
    self.0.get_import(namespace, operation)
  }

  pub(crate) fn get_export(&self, namespace: &str, operation: &str) -> Option<u32> {
    self.0.get_export(namespace, operation)
  }

  pub(crate) fn get_operation_list(&self) -> OperationList {
    self.0.get_operation_list()
  }
}

/// All engine providers must implement the [EngineProvider] trait.
#[async_trait::async_trait]
pub trait EngineProvider {
  /// Called to create a new [SharedContext].
  async fn new_context(&self, state: Arc<WasmSocket<HostServer>>) -> Result<SharedContext>;
}

/// The trait implemented by a context for a call or set of calls.
#[async_trait::async_trait]
pub trait ProviderCallContext: wasmrs::ModuleHost {
  /// Initialize the call context.
  async fn init(&self, host_buffer_size: u32, guest_buffer_size: u32) -> Result<()>;
}

/// The trait that a host needs to implement to satisfy wasmrs protocol imports and to query data about the loaded module.
pub trait CallbackProvider {
  /// The callback for `__wasmrs_send`
  fn do_host_send(&self, frame_bytes: Bytes) -> Result<()>;
  #[allow(missing_docs)]
  fn do_console_log(&self, msg: &str);
  /// Query the operation list for the module.
  fn do_op_list(&mut self, bytes: Bytes) -> Result<()>;
  /// The callback for `__wasmrs_init`
  fn do_host_init(&self, guest_buff_ptr: u32, host_buff_ptr: u32) -> Result<()>;
}
