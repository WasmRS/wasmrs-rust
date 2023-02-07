use std::sync::Arc;

use bytes::Bytes;
use parking_lot::Mutex;
use wasmrs::{BufferState, Frame, OperationList, WasmSocket};
use wasmrs_host::errors::Error;
use wasmrs_host::{CallbackProvider, WasiParams};
use wasmtime::{Engine, Store};

use crate::wasi::init_wasi;

type WasiCtx = wasmtime_wasi::WasiCtx;

pub(crate) struct ProviderStore {
  pub(crate) wasi_ctx: Option<WasiCtx>,
  pub(crate) socket: Arc<WasmSocket>,
  pub(crate) host_buffer: BufferState,
  pub(crate) guest_buffer: BufferState,
  pub(crate) op_list: Arc<Mutex<OperationList>>,
}

impl CallbackProvider for ProviderStore {
  fn do_host_init(&self, guest_buff_ptr: u32, host_buff_ptr: u32) -> Result<(), Error> {
    self.host_buffer.update_start(host_buff_ptr);
    self.guest_buffer.update_start(guest_buff_ptr);
    Ok(())
  }

  fn do_host_send(&self, frame_bytes: Bytes) -> Result<(), Error> {
    match Frame::decode(frame_bytes) {
      Ok(frame) => {
        self.socket.send(frame);
        Ok(())
      }
      Err((stream_id, err)) => {
        self.socket.send(Frame::new_error(stream_id, 0, err.to_string()));
        Ok(())
      }
    }
  }

  fn do_console_log(&self, msg: &str) {
    println!("{}", msg);
  }

  fn do_op_list(&self, bytes: Bytes) -> Result<(), Error> {
    let op_list = match OperationList::decode(bytes) {
      Ok(v) => v,
      Err(e) => {
        eprintln!("Could not decode operation list: {}", e);
        return Err(Error::OpList(e.to_string()));
      }
    };
    *self.op_list.lock() = op_list;
    Ok(())
  }

  fn get_import(&self, namespace: &str, operation: &str) -> Result<u32, Error> {
    self
      .op_list
      .lock()
      .get_import(namespace, operation)
      .ok_or_else(|| Error::OpMissing(namespace.to_owned(), operation.to_owned()))
  }
  fn get_export(&self, namespace: &str, operation: &str) -> Result<u32, Error> {
    self
      .op_list
      .lock()
      .get_export(namespace, operation)
      .ok_or_else(|| Error::OpMissing(namespace.to_owned(), operation.to_owned()))
  }
}

pub(crate) fn new_store(
  wasi_params: &Option<WasiParams>,
  socket: Arc<WasmSocket>,
  engine: &Engine,
) -> super::Result<Store<ProviderStore>> {
  let params = wasi_params.clone().unwrap_or_default();
  let ctx = init_wasi(&params)?;
  Ok(Store::new(
    engine,
    ProviderStore {
      host_buffer: Default::default(),
      guest_buffer: Default::default(),
      op_list: Arc::new(Mutex::new(OperationList::default())),
      socket,
      wasi_ctx: Some(ctx),
    },
  ))
}
