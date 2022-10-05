use std::sync::Arc;

use bytes::Bytes;
use parking_lot::Mutex;
use wasmrs::{Frame, OperationList, WasmSocket};

type Result<T> = std::result::Result<T, crate::errors::Error>;

#[derive(Clone)]
#[allow(missing_debug_implementations)]
pub struct SharedContext(Arc<Mutex<dyn ProviderCallContext + Send + Sync + 'static>>);

impl SharedContext {
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

pub trait EngineProvider {
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    fn new_context(&self, state: Arc<WasmSocket>) -> Result<SharedContext>;
}

pub trait ProviderCallContext: wasmrs::ModuleHost {
    fn init(&mut self) -> Result<()>;
}

pub trait CallbackProvider {
    fn do_host_send(&self, frame_bytes: Bytes) -> Result<()>;
    fn do_console_log(&self, msg: &str);
    fn do_op_list(&self, bytes: Bytes) -> Result<()>;
    fn do_host_init(&self, guest_buff_ptr: u32, host_buff_ptr: u32) -> Result<()>;
    fn get_import(&self, namespace: &str, operation: &str) -> Result<u32>;
    fn get_export(&self, namespace: &str, operation: &str) -> Result<u32>;
}
