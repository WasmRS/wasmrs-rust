use std::sync::Arc;

use bytes::Bytes;
use wasmrs::{BufferState, Frame, OperationList, PayloadError, RSocket, WasmSocket};
use wasmrs_host::CallbackProvider;
use wasmrs_host::{errors::Error, HostServer};
use wasmtime::{Engine, Store};

type WasiCtx = wasmtime_wasi::WasiCtx;

pub(crate) struct ProviderStore<T> {
  pub(crate) wasi_ctx: Option<WasiCtx>,
  pub(crate) socket: Arc<WasmSocket<T>>,
  pub(crate) host_buffer: BufferState,
  pub(crate) guest_buffer: BufferState,
  pub(crate) op_list: OperationList,
}

impl<T: RSocket> CallbackProvider for ProviderStore<T> {
  fn do_host_init(&self, guest_buff_ptr: u32, host_buff_ptr: u32) -> Result<(), Error> {
    self.guest_buffer.update_start(guest_buff_ptr);
    self.host_buffer.update_start(host_buff_ptr);
    Ok(())
  }

  fn do_host_send(&self, frame_bytes: Bytes) -> Result<(), Error> {
    match Frame::decode(frame_bytes) {
      Ok(frame) => self
        .socket
        .process_once(frame)
        .map_err(|e| Error::SendFailed(e.to_string())),
      Err((stream_id, err)) => {
        self
          .socket
          .send(Frame::new_error(stream_id, PayloadError::new(0, err.to_string(), None)));
        Ok(())
      }
    }
  }

  fn do_console_log(&self, msg: &str) {
    println!("{}", msg);
  }

  fn do_op_list(&mut self, bytes: Bytes) -> Result<(), Error> {
    let op_list = match OperationList::decode(bytes) {
      Ok(v) => v,
      Err(e) => {
        eprintln!("Could not decode operation list: {}", e);
        return Err(Error::OpList(e.to_string()));
      }
    };
    self.op_list = op_list;
    Ok(())
  }
}

pub(crate) fn new_store(
  wasi_ctx: Option<WasiCtx>,
  socket: Arc<WasmSocket<HostServer>>,
  engine: &Engine,
) -> super::Result<Store<ProviderStore<HostServer>>> {
  Ok(Store::new(
    engine,
    ProviderStore {
      host_buffer: Default::default(),
      guest_buffer: Default::default(),
      op_list: OperationList::default(),
      socket,
      wasi_ctx,
    },
  ))
}
