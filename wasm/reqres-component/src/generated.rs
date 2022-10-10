use wasmrs_guest::select_all;
use wasmrs_guest::FutureExt;
use wasmrs_guest::StreamExt;

use crate::guest::*;

pub(crate) type GEN_RC_INPUTS = FluxReceiver<String, PayloadError>;

pub(crate) type GEN_RC_OUTPUTS = Flux<String, PayloadError>;

pub(crate) struct GEN_RC {}

impl RequestChannel for GEN_RC {
    fn request_channel_wrapper(input: IncomingStream) -> Result<OutgoingStream, GenericError> {
        // generated
        let (inputs_tx, inputs_rx) = Flux::<String, PayloadError>::new_parts();

        spawn(async move {
            while let Ok(Some(Ok(payload))) = input.recv().await {
                inputs_tx.send_result(deserialize(&payload.data).map_err(|e| e.into()));
            }
        });
        let (real_out_tx, real_out_rx) = Flux::new_parts();
        let (outputs_tx, mut outputs_rx) = Flux::new_parts();

        spawn(async move {
            while let Some(result) = outputs_rx.next().await {
                match result {
                    Ok(payload) => match serialize(&payload) {
                        Ok(bytes) => {
                            real_out_tx.send(Payload::new_optional(None, Some(Bytes::from(bytes))));
                        }
                        Err(e) => {
                            real_out_tx.error(PayloadError::application_error(e.to_string()));
                        }
                    },
                    Err(err) => {
                        real_out_tx.error(err);
                    }
                }
            }
        });

        spawn(async move {
            let _result = Self {}.task(inputs_rx, outputs_tx).await;
        });

        Ok(real_out_rx)
    }
}

pub(crate) type GEN_RS_INPUTS = Mono<String, PayloadError>;

pub(crate) type GEN_RS_OUTPUTS = Flux<String, PayloadError>;

pub(crate) struct GEN_RS {}

impl RequestStream for GEN_RS {
    fn request_stream_wrapper(input: IncomingMono) -> Result<OutgoingStream, GenericError> {
        // generated

        let (out_tx, out_rx) = Flux::new_parts();

        let input = Mono::from_future(async move {
            match input.await {
                Ok(bytes) => match deserialize(&bytes.data) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PayloadError::application_error(e.to_string())),
                },
                Err(e) => Err(PayloadError::application_error(e.to_string())),
            }
        });

        spawn(async move {
            let task = Self {};
            let (outputs_tx, mut outputs_rx) = Flux::new_parts();
            let outputs = outputs_tx;
            match task.task(input, outputs).await {
                Ok(_) => {
                    while let Some(next) = outputs_rx.next().await {
                        let out = match next {
                            Ok(output) => match serialize(&output) {
                                Ok(bytes) => Ok(Payload::new_optional(None, Some(bytes.into()))),
                                Err(e) => Err(PayloadError::application_error(e.to_string())),
                            },
                            Err(e) => Err(e),
                        };
                        let _ = out_tx.send_result(out);
                    }
                    out_tx.complete();
                }
                Err(e) => {
                    let _ = out_tx.error(PayloadError::application_error(e.to_string()));
                }
            };
        });

        Ok(out_rx)
    }
}

pub(crate) type GEN_RR_INPUTS = Mono<String, PayloadError>;

pub(crate) type GEN_RR_OUTPUTS = Mono<String, PayloadError>;

pub(crate) struct GEN_RR {}

impl RequestResponse for GEN_RR {
    fn request_response_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
        let (tx, rx) = runtime::oneshot();

        let input = Mono::from_future(async move {
            match input.await {
                Ok(bytes) => match deserialize(&bytes.data) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PayloadError::application_error(e.to_string())),
                },
                Err(e) => Err(PayloadError::application_error(e.to_string())),
            }
        });

        spawn(async move {
            let task = Self {};
            let output = Mono::new();
            let output = match task.task(input, output).await {
                Ok(output) => match output.await {
                    Ok(output) => match serialize(&output) {
                        Ok(bytes) => Ok(Payload::new_optional(None, Some(bytes.into()))),
                        Err(e) => Err(PayloadError::application_error(e.to_string())),
                    },
                    Err(e) => Err(e),
                },
                Err(e) => Err(PayloadError::application_error(e.to_string())),
            };
            let _ = tx.send(output);
        });

        Ok(Mono::from_future(async move {
            rx.await
                .map_err(|e| PayloadError::application_error(e.to_string()))?
        }))
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub(crate) struct GEN_TG_RR_INPUTS {
    pub(crate) firstName: String,
    pub(crate) lastName: String,
}

pub(crate) type GEN_TG_RR_OUTPUTS = String;

pub(crate) struct GEN_TG_RR {}

impl RequestResponse for GEN_TG_RR {
    fn request_response_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
        let (tx, rx) = runtime::oneshot();

        let input = Mono::from_future(async move {
            match input.await {
                Ok(bytes) => {
                    println!("got bytes: {:?}", bytes.data);
                    match deserialize(&bytes.data) {
                        Ok(v) => Ok(v),
                        Err(e) => Err(PayloadError::application_error(e.to_string())),
                    }
                }
                Err(e) => Err(PayloadError::application_error(e.to_string())),
            }
        });

        spawn(async move {
            let task = Self {};
            let output = Mono::new();
            let output = match task.task(input, output).await {
                Ok(output) => match output.await {
                    Ok(output) => match serialize(&output) {
                        Ok(bytes) => Ok(Payload::new_optional(None, Some(bytes.into()))),
                        Err(e) => Err(PayloadError::application_error(e.to_string())),
                    },
                    Err(e) => Err(e),
                },
                Err(e) => Err(PayloadError::application_error(e.to_string())),
            };
            let _ = tx.send(output);
        });

        Ok(Mono::from_future(async move {
            rx.await
                .map_err(|e| PayloadError::application_error(e.to_string()))?
        }))
    }
}
