use std::{cell::UnsafeCell, rc::Rc};

use futures_util::StreamExt;
use wasmrs::{flux::*, flux_try, runtime, Payload, PayloadError, RSocket};

use crate::{error::Error, OperationMap, ProcessFactory};

#[allow(missing_debug_implementations, missing_copy_implementations)]
pub(crate) struct WasmServer {}

impl RSocket for WasmServer {
    fn fire_and_forget(&self, _req: Payload) -> Mono<(), PayloadError> {
        todo!()
    }

    fn request_response(&self, payload: Payload) -> FluxReceiver<Payload, PayloadError> {
        let (tx, rx) = Flux::new_parts();

        let metadata = flux_try!(payload.parse_metadata());

        let handler = flux_try!(get_process_handler(
            &crate::guest::REQUEST_RESPONSE_HANDLERS,
            metadata.index as _,
        ));

        let outgoing = flux_try!(handler(rx).map_err(|e| Error::HandlerFail(e.to_string())));
        let _ = tx.send(flux_try!(payload.try_into()));
        tx.complete();

        outgoing.split_receiver().unwrap()
    }

    fn request_stream(&self, payload: Payload) -> FluxReceiver<Payload, PayloadError> {
        let (tx, rx) = Flux::new_parts();

        let metadata = flux_try!(payload.parse_metadata());

        let handler = flux_try!(get_process_handler(
            &crate::guest::REQUEST_RESPONSE_HANDLERS,
            metadata.index as _,
        ));

        let outgoing = flux_try!(handler(rx).map_err(|e| Error::HandlerFail(e.to_string())));
        flux_try!(tx.send(flux_try!(payload.try_into())));
        tx.complete();

        outgoing.split_receiver().unwrap()
    }

    fn request_channel(
        &self,
        mut stream: FluxReceiver<Payload, PayloadError>,
    ) -> FluxReceiver<Payload, PayloadError> {
        let (tx, rx) = Flux::new_parts();

        runtime::spawn(async move {
            let (handler_input, handler_stream) = Flux::new_parts();
            let mut handler_out = if let Some(result) = stream.next().await {
                let payload = flux_try!(tx, result);

                let metadata = flux_try!(tx, payload.parse_metadata());
                let handler = flux_try!(
                    tx,
                    get_process_handler(
                        &crate::guest::REQUEST_RESPONSE_HANDLERS,
                        metadata.index as _,
                    )
                );

                handler_input.send(payload.try_into().unwrap()).unwrap();
                flux_try!(
                    tx,
                    handler(handler_stream).map_err(|e| Error::HandlerFail(e.to_string()))
                )
            } else {
                let _ = tx.error(PayloadError::application_error(
                    "Can not initiate a channel with no payload",
                ));
                return;
            };
            runtime::spawn(async move {
                while let Some(payload) = handler_out.next().await {
                    let _ = tx.send_result(payload);
                }
                tx.complete();
            });
            while let Some(next) = stream.next().await {
                let _ = handler_input.send_result(next.and_then(|v| {
                    v.try_into()
                        .map_err(|e: Error| PayloadError::application_error(e.to_string()))
                }));
            }
        });

        rx
    }
}

fn get_process_handler(
    kind: &'static std::thread::LocalKey<UnsafeCell<OperationMap>>,
    index: usize,
) -> Result<Rc<ProcessFactory>, Error> {
    kind.with(|cell| {
        #[allow(unsafe_code)]
        let buffer = unsafe { &*cell.get() };
        buffer
            .get(index)
            .map(|(_, _, op)| op.clone())
            .ok_or(Error::NoHandler)
    })
}
