use std::error::Error;
use std::pin::Pin;
use std::sync::Arc;

use futures_core::Stream;
use parking_lot::Mutex;

use crate::host::modulestate::ModuleState;
use crate::Invocation;

pub trait WebAssemblyEngineProvider {
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    fn new_context(
        &self,
        state: Arc<ModuleState>,
    ) -> Result<Arc<Mutex<dyn ProviderCallContext + Sync + Send>>>;
}

type Result<T> = std::result::Result<T, crate::errors::Error>;

pub trait ProviderCallContext: wasmrs_rsocket::FrameWriter {
    fn init(&mut self) -> Result<()>;
}
