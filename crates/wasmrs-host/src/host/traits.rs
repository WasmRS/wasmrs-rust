use std::error::Error;
use std::sync::Arc;

use crate::host::modulestate::ModuleState;
use crate::Invocation;

/// The module host (wasmRS) must provide an implementation of this trait to the engine provider
/// to enable wasmRS function calls.
pub trait ModuleHost {
    /// Called by the engine provider to obtain the Invocation bound for the guest module
    fn get_guest_request(&self) -> Option<Invocation>;

    /// Called by the engine provider to query the results of a host function call
    fn get_host_response(&self) -> Option<Vec<u8>>;

    /// Called by the engine provider to set the error message indicating a failure that occurred inside the guest module execution
    fn set_guest_error(&self, error: String);

    /// Called by the engine provider to set the response data for a guest call
    fn set_guest_response(&self, response: Vec<u8>);

    /// Called by the engine provider to query the host error if one is indicated by the return code for a host call
    fn get_host_error(&self) -> Option<String>;

    /// Sets the value indicating the response data from a guest call
    fn set_guest_call_complete(&self, index: i32, code: i32, response: Vec<u8>);

    /// Called by the engine provider to allow a guest module to perform a host call. The numeric return value
    /// will be > 0 for success (engine must obtain the host response) or 0 for error (engine must obtain the error)
    fn do_host_call(
        &self,
        binding: &str,
        namespace: &str,
        operation: &str,
        payload: &[u8],
    ) -> Result<i32, Box<dyn Error>>;

    /// Attempts to perform a console log. There are no guarantees this will happen, and no error will be returned
    /// to the guest module if the host rejects the attempt
    fn do_console_log(&self, msg: &str);
}

/// An engine provider is any code that encapsulates low-level WebAssembly interactions such
/// as reading from and writing to linear memory, executing functions, and mapping imports
/// in a way that conforms to the wasmRS conversation protocol.
pub trait WebAssemblyEngineProvider {
    /// Tell the engine provider that it can do whatever processing it needs to do for
    /// initialization and give it access to the module state
    fn init(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(())
    }

    /// Tell the engine provider that it can do whatever processing it needs to do for
    /// initialization and give it access to the module state
    fn new_context(
        &self,
        state: Arc<ModuleState>,
    ) -> Result<Box<dyn ProviderCallContext + Sync + Send>, Box<dyn Error + Send + Sync>>;
}

/// A context that we can call functions from.
pub trait ProviderCallContext {
    /// Tell the engine provider that it can do whatever processing it needs to do for
    /// initialization and give it access to the module state
    fn init(&mut self) -> Result<(), Box<dyn Error + Send + Sync>>;

    fn request_response(
        &mut self,
        stream_id: u32,
        payload: Vec<u8>,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
}
