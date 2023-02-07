use std::cell::UnsafeCell;
use std::rc::Rc;

use futures_util::StreamExt;
use wasmrs::{Payload, RSocket};
use wasmrs_frames::PayloadError;
use wasmrs_runtime as runtime;
use wasmrs_rx::*;

use crate::error::Error;
use crate::{OperationMap, ParsedPayload};

macro_rules! flux_try {
  ($expr:expr) => {{
    match $expr {
      Ok(v) => v,
      Err(e) => {
        let flux = Flux::new();
        let _ = flux.error(PayloadError::application_error(e.to_string()));
        return flux.take_rx().unwrap();
      }
    }
  }};
  ($tx:ident, $expr:expr) => {{
    match $expr {
      Ok(v) => v,
      Err(e) => {
        let _ = $tx.error(PayloadError::application_error(e.to_string()));
        return;
      }
    }
  }};
}

macro_rules! mono_try {
  ($expr:expr) => {{
    match $expr {
      Ok(v) => v,
      Err(e) => return Mono::new_error(PayloadError::application_error(e.to_string())),
    }
  }};
}

#[allow(missing_debug_implementations, missing_copy_implementations)]
pub(crate) struct WasmServer {}

impl RSocket for WasmServer {
  fn fire_and_forget(&self, payload: Payload) -> Mono<(), PayloadError> {
    let metadata = mono_try!(payload.parse_metadata());

    let handler = mono_try!(get_process_handler(
      &crate::guest::REQUEST_FNF_HANDLERS,
      metadata.index as _,
    ));

    let parsed: ParsedPayload = mono_try!(payload.try_into());

    mono_try!(handler(Mono::new_success(parsed)).map_err(|e| Error::HandlerFail(e.to_string())));

    Mono::new_success(())
  }

  fn request_response(&self, payload: Payload) -> Mono<Payload, PayloadError> {
    let metadata = mono_try!(payload.parse_metadata());

    let handler = mono_try!(get_process_handler(
      &crate::guest::REQUEST_RESPONSE_HANDLERS,
      metadata.index as _,
    ));

    let parsed: ParsedPayload = mono_try!(payload.try_into());

    mono_try!(handler(Mono::new_success(parsed)).map_err(|e| Error::HandlerFail(e.to_string())))
  }

  fn request_stream(&self, payload: Payload) -> FluxReceiver<Payload, PayloadError> {
    let metadata = flux_try!(payload.parse_metadata());

    let handler = flux_try!(get_process_handler(
      &crate::guest::REQUEST_STREAM_HANDLERS,
      metadata.index as _,
    ));

    let parsed: ParsedPayload = flux_try!(payload.try_into());
    let mono = Mono::new_success(parsed);

    flux_try!(handler(mono).map_err(|e| Error::HandlerFail(e.to_string())))
  }

  fn request_channel(&self, mut stream: FluxReceiver<Payload, PayloadError>) -> FluxReceiver<Payload, PayloadError> {
    let (tx, rx) = Flux::new_channels();

    runtime::spawn(async move {
      let (handler_input, handler_stream) = Flux::new_channels();
      let mut handler_out = if let Some(result) = stream.next().await {
        let payload = flux_try!(tx, result);

        let metadata = flux_try!(tx, payload.parse_metadata());
        let handler = flux_try!(
          tx,
          get_process_handler(&crate::guest::REQUEST_CHANNEL_HANDLERS, metadata.index as _,)
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

fn get_process_handler<T>(
  kind: &'static std::thread::LocalKey<UnsafeCell<OperationMap<T>>>,
  index: usize,
) -> Result<Rc<T>, Error> {
  kind.with(|cell| {
    #[allow(unsafe_code)]
    let buffer = unsafe { &*cell.get() };
    buffer.get(index).map(|(_, _, op)| op.clone()).ok_or(Error::NoHandler)
  })
}
