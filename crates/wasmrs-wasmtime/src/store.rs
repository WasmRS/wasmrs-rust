use wasmrs_host::WasiParams;
use wasmtime::{Engine, Store};

use crate::wasi::init_wasi;

type WasiCtx = wasmtime_wasi::WasiCtx;

pub(crate) struct WasmRsStore {
    pub(crate) wasi_ctx: Option<WasiCtx>,
}

pub(crate) fn new_store(
    wasi_params: &Option<WasiParams>,
    engine: &Engine,
) -> super::Result<Store<WasmRsStore>> {
    trace!("creating new memory store");
    let params = wasi_params.clone().unwrap_or_default();
    let ctx = init_wasi(&params)?;
    Ok(Store::new(
        engine,
        WasmRsStore {
            wasi_ctx: Some(ctx),
        },
    ))
}
