use bytes::Bytes;
use wasmrs_host::{CallbackProvider, HostExports, IntoEnumIterator};
use wasmtime::{AsContext, Caller, FuncType, Linker, Val, ValType};

use crate::errors::Error;
use crate::memory::{get_caller_memory, get_vec_from_memory, read_frame};
use crate::store::ProviderStore;

pub(crate) fn add_to_linker(linker: &mut Linker<ProviderStore>) -> super::Result<()> {
  let module_name = wasmrs_host::HOST_NAMESPACE;
  for export in HostExports::iter() {
    match export {
      HostExports::Send => {
        let (extern_type, extern_fn) = linker_send();
        linker
          .func_new(module_name, export.as_ref(), extern_type, extern_fn)
          .map_err(Error::Func)?;
      }
      HostExports::Init => {
        let (extern_type, extern_fn) = linker_init();
        linker
          .func_new(module_name, export.as_ref(), extern_type, extern_fn)
          .map_err(Error::Func)?;
      }
      HostExports::Log => {
        let (extern_type, extern_fn) = linker_console_log();
        linker
          .func_new(module_name, export.as_ref(), extern_type, extern_fn)
          .map_err(Error::Func)?;
      }
      HostExports::OpList => {
        let (extern_type, extern_fn) = linker_op_list();
        linker
          .func_new(module_name, export.as_ref(), extern_type, extern_fn)
          .map_err(Error::Func)?;
      }
    };
  }
  Ok(())
}

fn linker_send() -> (
  FuncType,
  impl Fn(Caller<'_, ProviderStore>, &[Val], &mut [Val]) -> Result<(), anyhow::Error> + Send + Sync + 'static,
) {
  (
    FuncType::new(vec![ValType::I32], vec![]),
    move |mut caller, params: &[Val], _results: &mut [Val]| {
      trace!(
        import = wasmrs_host::HostExports::Send.as_ref(),
        ?params,
        "guest calling host"
      );

      let read_until = params[0].unwrap_i32() as usize;
      let memory = get_caller_memory(&mut caller);
      let bytes = read_frame(
        caller.as_context(),
        memory,
        caller.data().host_buffer.get_start() as _,
        read_until,
      )?;
      trace!(?bytes, "got frame");

      caller.data().do_host_send(bytes)?;
      Ok(())
    },
  )
}

fn linker_init() -> (
  FuncType,
  impl Fn(Caller<'_, ProviderStore>, &[Val], &mut [Val]) -> Result<(), anyhow::Error> + Send + Sync + 'static,
) {
  (
    FuncType::new(vec![ValType::I32, ValType::I32], vec![]),
    move |caller, params: &[Val], _results: &mut [Val]| {
      trace!(
        import = wasmrs_host::HostExports::Init.as_ref(),
        ?params,
        "guest calling host"
      );

      let guest_buff_ptr = params[0].unwrap_i32();
      let host_buff_ptr = params[1].unwrap_i32();

      caller
        .data()
        .do_host_init(guest_buff_ptr.try_into().unwrap(), host_buff_ptr.try_into().unwrap())?;

      Ok(())
    },
  )
}

fn linker_console_log() -> (
  FuncType,
  impl Fn(Caller<'_, ProviderStore>, &[Val], &mut [Val]) -> Result<(), anyhow::Error> + Send + Sync + 'static,
) {
  (
    FuncType::new(vec![ValType::I32, ValType::I32], vec![]),
    move |mut caller, params: &[Val], _results: &mut [Val]| {
      let ptr = params[0].i32();
      let len = params[1].i32();
      let memory = get_caller_memory(&mut caller);
      let vec = get_vec_from_memory(caller.as_context(), memory, ptr.unwrap(), len.unwrap());

      let msg = std::str::from_utf8(&vec).unwrap();

      caller.data().do_console_log(msg);
      Ok(())
    },
  )
}

fn linker_op_list() -> (
  FuncType,
  impl Fn(Caller<'_, ProviderStore>, &[Val], &mut [Val]) -> Result<(), anyhow::Error> + Send + Sync + 'static,
) {
  (
    FuncType::new(vec![ValType::I32, ValType::I32], vec![]),
    move |mut caller, params: &[Val], _results: &mut [Val]| {
      trace!(
          import = %wasmrs_host::HostExports::OpList,
          ?params,
          "guest calling host"
      );
      let ptr = params[0].i32();
      let len = params[1].i32();
      let memory = get_caller_memory(&mut caller);
      let vec = get_vec_from_memory(caller.as_context(), memory, ptr.unwrap(), len.unwrap());

      caller.data().do_op_list(Bytes::from(vec))?;
      Ok(())
    },
  )
}
