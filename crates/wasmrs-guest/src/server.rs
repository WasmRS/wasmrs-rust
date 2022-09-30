use std::{cell::UnsafeCell, rc::Rc};

use futures_util::StreamExt;
use wasmrs::{flux::*, flux_try, runtime, Payload, PayloadError, RSocket};

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
            flux_try!(handler(flux.split_receiver().unwrap())
                .map_err(|e| Error::HandlerFail(e.to_string())));
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
            flux_try!(handler(flux.split_receiver().unwrap())
                .map_err(|e| Error::HandlerFail(e.to_string())));
        let _ = flux.send(flux_try!(payload.try_into()));
        flux.complete();

        outgoing.split_receiver().unwrap()
    }

    fn request_channel(
        &self,
        mut stream: FluxReceiver<Payload, PayloadError>,
    ) -> FluxReceiver<Payload, PayloadError> {
        let flux = Flux::new();
        let outgoing = flux.split_receiver().unwrap();

        runtime::spawn(async move {
            let handler_input = Flux::new();
            let mut handler_out = if let Some(payload) = stream.next().await {
                match payload {
                    Ok(payload) => {
                        let metadata = match payload.parse_metadata() {
                            Ok(m) => m,
                            Err(_e) => {
                                return flux
                                    .error(PayloadError::application_error(
                                        "Could not parse metadata",
                                    ))
                                    .unwrap();
                            }
                        };
                        let handler_stream = handler_input.split_receiver().unwrap();
                        let handler = get_process_handler(
                            &crate::guest::REQUEST_RESPONSE_HANDLERS,
                            &metadata.namespace,
                            &metadata.operation,
                        );
                        let handler = match handler {
                            Ok(h) => h,
                            Err(_e) => {
                                return flux
                                    .error(PayloadError::application_error(
                                        "Could not find handler",
                                    ))
                                    .unwrap();
                            }
                        };
                        let result =
                            handler(handler_stream).map_err(|e| Error::HandlerFail(e.to_string()));
                        if result.is_err() {
                            return flux
                                .error(PayloadError::application_error("Handler failed"))
                                .unwrap();
                        }
                        handler_input.send(payload.try_into().unwrap()).unwrap();
                        result.unwrap()
                    }
                    Err(_e) => {
                        return flux
                            .error(PayloadError::application_error(
                                "Can not initiate a channel with an error",
                            ))
                            .unwrap()
                    }
                }
            } else {
                return flux
                    .error(PayloadError::application_error(
                        "Can not initiate a channel with no payload",
                    ))
                    .unwrap();
            };
            runtime::spawn(async move {
                while let Some(payload) = handler_out.next().await {
                    let _ = flux.send_result(payload);
                }
                flux.complete();
            });
            while let Some(next) = stream.next().await {
                handler_input
                    .send_result(next.and_then(|v| {
                        v.try_into()
                            .map_err(|e: Error| PayloadError::application_error(e.to_string()))
                    }))
                    .unwrap();
            }
        });

        outgoing
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
