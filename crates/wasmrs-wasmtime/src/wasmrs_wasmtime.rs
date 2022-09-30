use std::sync::Arc;

use wasmrs::WasmSocket;
use wasmrs_host::{HostExports, IntoEnumIterator};
use wasmtime::{AsContext, Caller, FuncType, Linker, Trap, Val, ValType};

use crate::{
    memory::{get_caller_memory, get_vec_from_memory, read_frame},
    store::ProviderStore,
};

pub(crate) fn add_to_linker(
    linker: &mut Linker<ProviderStore>,
    host: &Arc<WasmSocket>,
) -> super::Result<()> {
    let module_name = wasmrs_host::HOST_NAMESPACE;
    for export in HostExports::iter() {
        match export {
            HostExports::Send => {
                let (extern_type, extern_fn) = linker_send(host.clone());
                linker.func_new(module_name, export.as_ref(), extern_type, extern_fn)?;
            }
            HostExports::Init => {
                let (extern_type, extern_fn) = linker_init(host.clone());
                linker.func_new(module_name, export.as_ref(), extern_type, extern_fn)?;
            }
            HostExports::Log => {
                let (extern_type, extern_fn) = linker_console_log(host.clone());
                linker.func_new(module_name, export.as_ref(), extern_type, extern_fn)?;
            }
        };
    }
    Ok(())
}

fn linker_send(
    host: Arc<WasmSocket>,
) -> (
    FuncType,
    impl Fn(Caller<'_, ProviderStore>, &[Val], &mut [Val]) -> Result<(), Trap> + Send + Sync + 'static,
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
                host.host_buffer().get_start() as _,
                read_until,
            )
            .map_err(|e| wasmtime::Trap::new(e.to_string()))?;

            host.do_host_send(bytes)
                .map_err(|e| wasmtime::Trap::new(e.to_string()))?;
            Ok(())
        },
    )
}

fn linker_init(
    host: Arc<WasmSocket>,
) -> (
    FuncType,
    impl Fn(Caller<'_, ProviderStore>, &[Val], &mut [Val]) -> Result<(), Trap> + Send + Sync + 'static,
) {
    (
        FuncType::new(vec![ValType::I32, ValType::I32], vec![]),
        move |_caller, params: &[Val], _results: &mut [Val]| {
            trace!(
                import = wasmrs_host::HostExports::Init.as_ref(),
                ?params,
                "guest calling host"
            );

            let guest_buff_ptr = params[0].unwrap_i32();
            let host_buff_ptr = params[1].unwrap_i32();

            host.do_host_init(
                guest_buff_ptr.try_into().unwrap(),
                host_buff_ptr.try_into().unwrap(),
            )
            .map_err(|e| wasmtime::Trap::new(e.to_string()))?;

            Ok(())
        },
    )
}

fn linker_console_log(
    host: Arc<WasmSocket>,
) -> (
    FuncType,
    impl Fn(Caller<'_, ProviderStore>, &[Val], &mut [Val]) -> Result<(), Trap> + Send + Sync + 'static,
) {
    (
        FuncType::new(vec![ValType::I32, ValType::I32], vec![]),
        move |mut caller, params: &[Val], _results: &mut [Val]| {
            let ptr = params[0].i32();
            let len = params[1].i32();
            let memory = get_caller_memory(&mut caller);
            let vec = get_vec_from_memory(caller.as_context(), memory, ptr.unwrap(), len.unwrap());

            let msg = std::str::from_utf8(&vec).unwrap();

            host.do_console_log(msg);
            Ok(())
        },
    )
}
