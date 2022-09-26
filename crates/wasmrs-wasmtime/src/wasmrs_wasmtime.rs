use std::sync::Arc;

use bytes::Bytes;
use wasmrs_host::{HostExports, IntoEnumIterator, ModuleState};
use wasmrs_ringbuffer::SharedReadOnlyRingBuffer;
use wasmtime::{AsContext, Caller, FuncType, Linker, Memory, StoreContext, Trap, Val, ValType};

use crate::{errors::Error, store::WasmRsStore};

fn get_vec_from_memory<'a, T: 'a>(
    store: impl Into<StoreContext<'a, T>>,
    mem: Memory,
    ptr: i32,
    len: i32,
) -> Vec<u8> {
    let data = mem.data(store);
    data[ptr as usize..(ptr + len) as usize].to_vec()
}

fn get_caller_memory<T>(caller: &mut Caller<T>) -> Memory {
    let memory = caller
        .get_export("memory")
        .map(|e| e.into_memory().unwrap());
    memory.unwrap()
}

pub(crate) fn get_vec_from_ringbuffer<'a, T: 'a>(
    store: impl Into<StoreContext<'a, T>>,
    mem: Memory,
    recv_pos: u32,
    ring_start: u32,
    ring_len: u32,
) -> super::Result<Bytes> {
    let data = mem.data(store);
    let buff = SharedReadOnlyRingBuffer::new(
        data,
        ring_start as usize,
        ring_len as usize,
        recv_pos as usize,
    );
    wasmrs_rsocket::read_frame(buff).map_err(|_| Error::GuestMemory)
}

pub(crate) fn write_bytes_to_memory(
    store: impl AsContext,
    memory: Memory,
    buffer: &[u8],
    recv_pos: u32,
    ring_start: u32,
    ring_len: u32,
) -> u32 {
    let len = buffer.len();
    let remaining: usize = (ring_len - recv_pos) as _;
    let start_offset = ring_start + recv_pos;

    #[allow(unsafe_code)]
    unsafe {
        let mut guest_ptr = memory.data_ptr(&store).offset(start_offset as isize);
        if len > remaining {
            guest_ptr.copy_from(buffer.as_ptr(), remaining);
            guest_ptr = memory.data_ptr(store).offset(ring_start as isize);
            guest_ptr.copy_from(buffer.as_ptr().add(remaining), len - remaining);
        } else {
            guest_ptr.copy_from(buffer.as_ptr(), len);
        }
    }
    len as u32
}

pub(crate) fn add_to_linker(
    linker: &mut Linker<WasmRsStore>,
    host: &Arc<ModuleState>,
) -> super::Result<()> {
    let module_name = "wasmrs";
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
    host: Arc<ModuleState>,
) -> (
    FuncType,
    impl Fn(Caller<'_, WasmRsStore>, &[Val], &mut [Val]) -> Result<(), Trap> + Send + Sync + 'static,
) {
    (
        FuncType::new(vec![ValType::I32], vec![]),
        move |mut caller, params: &[Val], _results: &mut [Val]| {
            trace!(
                import = wasmrs_host::HostExports::Send.as_ref(),
                ?params,
                "guest calling host"
            );
            let instant = std::time::SystemTime::now();
            println!(
                "<<Host: Getting frame at {:?}",
                instant
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            );

            let recv_pos = params[0].unwrap_i32() as u32;
            let memory = get_caller_memory(&mut caller);
            let bytes = get_vec_from_ringbuffer(
                caller.as_context(),
                memory,
                recv_pos,
                host.get_host_buffer_start(),
                host.get_host_buffer_size(),
            )
            .map_err(|e| wasmtime::Trap::new(e.to_string()))?;

            host.do_host_send(bytes)
                .map_err(|e| wasmtime::Trap::new(e.to_string()))?;
            Ok(())
        },
    )
}

fn linker_init(
    host: Arc<ModuleState>,
) -> (
    FuncType,
    impl Fn(Caller<'_, WasmRsStore>, &[Val], &mut [Val]) -> Result<(), Trap> + Send + Sync + 'static,
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
    host: Arc<ModuleState>,
) -> (
    FuncType,
    impl Fn(Caller<'_, WasmRsStore>, &[Val], &mut [Val]) -> Result<(), Trap> + Send + Sync + 'static,
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
