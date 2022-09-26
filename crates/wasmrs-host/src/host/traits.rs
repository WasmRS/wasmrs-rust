use std::error::Error;
use std::pin::Pin;
use std::sync::Arc;

use futures_core::Stream;
use parking_lot::Mutex;
use wasmrs::Frame;

use crate::host::modulestate::ModuleState;
use crate::Invocation;

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
    pub(crate) fn write_frame(&self, stream_id: u32, frame: Frame) -> Result<()> {
        let result = self.0.lock().write_frame(stream_id, frame);

        if let Err(e) = &result {
            error!("send request_response failed: {}", e);
        }
        Ok(result?)
    }
}

pub trait WebAssemblyEngineProvider {
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    fn new_context(&self, state: Arc<ModuleState>) -> Result<SharedContext>;
}

type Result<T> = std::result::Result<T, crate::errors::Error>;

pub trait ProviderCallContext: wasmrs::FrameWriter {
    fn init(&mut self) -> Result<()>;
}
