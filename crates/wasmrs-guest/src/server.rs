use std::cell::UnsafeCell;

use futures_util::{FutureExt, StreamExt};
use runtime::RtRc;
use wasmrs::{BoxFlux, BoxMono, OperationMap, Payload, RSocket, RawPayload};
use wasmrs_frames::PayloadError;
use wasmrs_runtime as runtime;
use wasmrs_rx::{FluxChannel, Observer};

use crate::error::Error;

#[allow(missing_debug_implementations, missing_copy_implementations)]
pub(crate) struct WasmServer {}

impl RSocket for WasmServer {
  fn fire_and_forget(&self, payload: RawPayload) -> BoxMono<(), PayloadError> {
    futures_util::future::ready(request_fnf(payload).map_err(|e| PayloadError::application_error(e.to_string(), None)))
      .boxed()
  }

  fn request_response(&self, payload: RawPayload) -> BoxMono<RawPayload, PayloadError> {
    match request_response(payload) {
      Ok(v) => v,
      Err(e) => futures_util::future::ready(Err(PayloadError::application_error(e.to_string(), None))).boxed(),
    }
  }

  fn request_stream(&self, payload: RawPayload) -> BoxFlux<RawPayload, PayloadError> {
    match request_stream(payload) {
      Ok(flux) => flux,
      Err(e) => futures_util::stream::iter([Err(PayloadError::application_error(e.to_string(), None))]).boxed(),
    }
  }

  fn request_channel(&self, stream: BoxFlux<RawPayload, PayloadError>) -> BoxFlux<RawPayload, PayloadError> {
    match request_channel(stream) {
      Ok(flux) => flux,
      Err(e) => futures_util::stream::iter([Err(PayloadError::application_error(e.to_string(), None))]).boxed(),
    }
  }
}

fn request_fnf(payload: RawPayload) -> Result<(), Error> {
  let parsed: Payload = payload.try_into()?;

  let handler = get_process_handler(&crate::guest::REQUEST_FNF_HANDLERS, parsed.metadata.index as _)?;

  handler(Box::pin(futures_util::future::ready(Ok(parsed)))).map_err(|e| Error::HandlerFail(e.to_string()))?;
  Ok(())
}

fn request_response(payload: RawPayload) -> Result<BoxMono<RawPayload, PayloadError>, Error> {
  let parsed: Payload = payload.try_into()?;

  let handler = get_process_handler(&crate::guest::REQUEST_RESPONSE_HANDLERS, parsed.metadata.index as _)?;

  handler(Box::pin(futures_util::future::ready(Ok(parsed)))).map_err(|e| Error::HandlerFail(e.to_string()))
}

fn request_stream(payload: RawPayload) -> Result<BoxFlux<RawPayload, PayloadError>, Error> {
  let parsed: Payload = payload.try_into()?;
  let handler = get_process_handler(&crate::guest::REQUEST_STREAM_HANDLERS, parsed.metadata.index as _)?;
  handler(futures_util::future::ready(Ok(parsed)).boxed()).map_err(|e| Error::HandlerFail(e.to_string()))
}

fn request_channel(stream: BoxFlux<RawPayload, PayloadError>) -> Result<BoxFlux<RawPayload, PayloadError>, Error> {
  let (tx, rx) = FluxChannel::new_parts();

  runtime::spawn("guest:server:request_channel", async move {
    match request_channel_inner(stream).await {
      Ok(mut res_stream) => {
        while let Some(r) = res_stream.next().await {
          let _ = tx.send_result(r);
        }
      }
      Err(e) => {
        let _ = tx.error(PayloadError::application_error(e.to_string(), None));
      }
    }
  });
  Ok(rx.boxed())
}

async fn request_channel_inner(
  mut incoming_stream: BoxFlux<RawPayload, PayloadError>,
) -> Result<BoxFlux<RawPayload, PayloadError>, Error> {
  let (handler_input, handler_stream) = FluxChannel::new_parts();
  let handler_out = if let Some(result) = incoming_stream.next().await {
    let payload = match result {
      Ok(v) => v,
      Err(e) => {
        return Ok(futures_util::stream::iter([Err(e)]).boxed());
      }
    };

    let parsed: Payload = payload.try_into()?;
    let handler = get_process_handler(&crate::guest::REQUEST_CHANNEL_HANDLERS, parsed.metadata.index as _)?;

    handler_input.send(parsed).unwrap();

    handler(handler_stream.boxed()).map_err(|e| Error::HandlerFail(e.to_string()))?
  } else {
    return Ok(
      futures_util::stream::iter([Err(PayloadError::application_error(
        "Can not initiate a channel with no payload",
        None,
      ))])
      .boxed(),
    );
  };

  runtime::spawn("guest:server:request_channel_inner", async move {
    while let Some(next) = incoming_stream.next().await {
      let v = next.and_then(|v: RawPayload| {
        v.try_into()
          .map_err(|e: wasmrs::Error| PayloadError::application_error(e.to_string(), None))
      });
      let _ = handler_input.send_result(v);
    }
  });
  Ok(handler_out)
}

fn get_process_handler<T>(
  kind: &'static std::thread::LocalKey<UnsafeCell<OperationMap<T>>>,
  index: usize,
) -> Result<RtRc<T>, Error> {
  kind.with(|cell| {
    #[allow(unsafe_code)]
    let buffer = unsafe { &*cell.get() };
    buffer.get(index).map(|(_, _, op)| op.clone()).ok_or(Error::NoHandler)
  })
}
