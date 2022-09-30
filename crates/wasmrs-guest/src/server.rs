use std::{cell::UnsafeCell, rc::Rc};

use wasmrs::{flux::*, flux_try, Payload, PayloadError, RSocket};

use crate::{error::Error, NamespaceMap, ProcessFactory};

#[allow(missing_debug_implementations, missing_copy_implementations)]
pub(crate) struct WasmServer {}

impl RSocket for WasmServer {
    fn fire_and_forget(&self, _req: Payload) -> FluxReceiver<(), PayloadError> {
        todo!()
    }

    fn request_response(&self, payload: Payload) -> FluxReceiver<Payload, PayloadError> {
        let flux = Flux::new();

        let metadata = flux_try!(payload.parse_metadata());

        let handler = flux_try!(get_process_handler(
            &crate::guest::REQUEST_RESPONSE_HANDLERS,
            &metadata.namespace,
            &metadata.operation,
        ));

        let outgoing =
            flux_try!(handler(flux.clone()).map_err(|e| Error::HandlerFail(e.to_string())));
        let _ = flux.send(flux_try!(payload.try_into()));
        flux.complete();

        outgoing.split_receiver().unwrap()
    }

    fn request_stream(&self, payload: Payload) -> FluxReceiver<Payload, PayloadError> {
        let flux = Flux::new();

        let metadata = flux_try!(payload.parse_metadata());

        let handler = flux_try!(get_process_handler(
            &crate::guest::REQUEST_RESPONSE_HANDLERS,
            &metadata.namespace,
            &metadata.operation,
        ));

        let outgoing =
            flux_try!(handler(flux.clone()).map_err(|e| Error::HandlerFail(e.to_string())));
        let _ = flux.send(flux_try!(payload.try_into()));
        flux.complete();

        outgoing.split_receiver().unwrap()
    }

    fn request_channel(
        &self,
        _reqs: FluxReceiver<Payload, PayloadError>,
    ) -> FluxReceiver<Payload, PayloadError> {
        todo!()
    }
}

fn get_process_handler(
    kind: &'static std::thread::LocalKey<UnsafeCell<NamespaceMap>>,
    namespace: &str,
    op: &str,
) -> Result<Rc<ProcessFactory>, Error> {
    kind.with(|cell| {
        #[allow(unsafe_code)]
        let buffer = unsafe { &*cell.get() };
        buffer
            .get(namespace)
            .and_then(|opmap| opmap.get(op).cloned())
            .ok_or(Error::NoHandler)
    })
}
