use wasi_common::WasiCtx;
use wasmtime_wasi::{ambient_authority, Dir};

use crate::errors::Error;

pub(crate) fn init_ctx(
  preopen_dirs: &[(String, Dir)],
  argv: &[String],
  env: &[(String, String)],
) -> Result<WasiCtx, Error> {
  let mut ctx_builder = wasmtime_wasi::WasiCtxBuilder::new();

  ctx_builder = ctx_builder.inherit_stdio().args(argv)?.envs(env)?;

  for (name, file) in preopen_dirs {
    ctx_builder = ctx_builder.preopened_dir(file.try_clone()?, name)?;
  }

  Ok(ctx_builder.build())
}

pub(crate) fn compute_preopen_dirs(
  dirs: &[String],
  map_dirs: &[(String, String)],
) -> Result<Vec<(String, Dir)>, Error> {
  let ambient_authority = ambient_authority();
  let mut preopen_dirs = Vec::new();

  for dir in dirs.iter() {
    preopen_dirs.push((dir.clone(), Dir::open_ambient_dir(dir, ambient_authority)?));
  }

  for (guest, host) in map_dirs.iter() {
    preopen_dirs.push((guest.clone(), Dir::open_ambient_dir(host, ambient_authority)?));
  }

  Ok(preopen_dirs)
}

pub(crate) fn init_wasi(params: &wasmrs_host::WasiParams) -> Result<WasiCtx, Error> {
  init_ctx(
    &compute_preopen_dirs(&params.preopened_dirs, &params.map_dirs).unwrap(),
    &params.argv,
    &params.env_vars,
  )
}
