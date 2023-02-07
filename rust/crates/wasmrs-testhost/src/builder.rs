use crate::engine_provider::EpochDeadlines;
use crate::errors::Error;
use crate::WasmtimeEngineProvider;

/// Used to build [`WasmtimeEngineProvider`](crate::WasmtimeEngineProvider) instances.
#[allow(missing_debug_implementations)]
#[must_use]
#[derive(Default)]
pub struct WasmtimeBuilder<'a> {
  engine: Option<wasmtime::Engine>,
  module_bytes: &'a [u8],
  cache_enabled: bool,
  cache_path: Option<std::path::PathBuf>,
  wasi_params: Option<wasmrs_host::WasiParams>,
  epoch_deadlines: Option<EpochDeadlines>,
}

#[allow(deprecated)]
impl<'a> WasmtimeBuilder<'a> {
  /// A new WasmtimeEngineProviderBuilder instance,
  /// must provide the wasm module to be loaded
  pub fn new(module_bytes: &'a [u8]) -> Self {
    WasmtimeBuilder {
      module_bytes,
      ..Default::default()
    }
  }

  /// Provide a preinitialized [`wasmtime::Engine`]
  ///
  /// **Warning:** when used, engine specific options like
  /// [`cache`](WasmtimeEngineProviderBuilder::enable_cache) and
  /// [`enable_epoch_interruptions`](WasmtimeEngineProviderBuilder::enable_epoch_interruptions)
  /// must be pre-configured by the user. `WasmtimeEngineProviderBuilder` won't be
  /// able to configure them at [`build`](WasmtimeEngineProviderBuilder::build) time.
  pub fn engine(mut self, engine: wasmtime::Engine) -> Self {
    self.engine = Some(engine);
    self
  }

  /// WASI params
  pub fn wasi_params(mut self, wasi: wasmrs_host::WasiParams) -> Self {
    self.wasi_params = Some(wasi);
    self
  }

  /// Enable Wasmtime cache feature
  ///
  /// **Warning:** this has no effect when a custom [`wasmtime::Engine`] is provided via
  /// the [`WasmtimeEngineProviderBuilder::engine`] helper. In that case, it's up to the
  /// user to provide a [`wasmtime::Engine`] instance with the cache values properly configured.
  pub fn enable_cache(mut self, path: Option<&std::path::Path>) -> Self {
    self.cache_enabled = true;
    self.cache_path = path.map(|p| p.to_path_buf());
    self
  }

  /// Enable Wasmtime [epoch-based interruptions](wasmtime::Config::epoch_interruption) and set
  /// the deadlines to be enforced
  ///
  /// Two kind of deadlines have to be set:
  ///
  /// * `wasmrs_init_deadline`: the number of ticks the wasmRS initialization code can take before the
  ///   code is interrupted. This is the code usually defined inside of the `wasmrs_init`/`_start`
  ///   functions
  /// * `wasmrs_func_deadline`: the number of ticks any regular wasmRS guest function can run before
  ///   its terminated by the host
  ///
  /// Both these limits are expressed using the number of ticks that are allowed before the
  /// WebAssembly execution is interrupted.
  /// It's up to the embedder of wasmRS to define how much time a single tick is granted. This could
  /// be 1 second, 10 nanoseconds, or whatever the user prefers.
  ///
  /// **Warning:** when providing an instance of `wasmtime::Engine` via the
  /// `WasmtimeEngineProvider::engine` helper, ensure the `wasmtime::Engine`
  /// has been created with the `epoch_interruption` feature enabled
  pub fn enable_epoch_interruptions(mut self, wasmrs_init_deadline: u64, wasmrs_func_deadline: u64) -> Self {
    self.epoch_deadlines = Some(EpochDeadlines {
      wasmrs_init: wasmrs_init_deadline,
      wasmrs_func: wasmrs_func_deadline,
    });
    self
  }

  /// Create a `WasmtimeEngineProvider` instance
  pub fn build(&self) -> Result<WasmtimeEngineProvider, Error> {
    let mut provider = match &self.engine {
      Some(e) => {
        // note: we have to call `.clone()` because `e` is behind
        // a shared reference and `Engine` does not implement `Copy`.
        // However, cloning an `Engine` is a cheap operation because
        // under the hood wasmtime does not create a new `Engine`, but
        // rather creates a new reference to it.
        // See https://docs.rs/wasmtime/latest/wasmtime/struct.Engine.html#engines-and-clone
        WasmtimeEngineProvider::new_with_engine(self.module_bytes, e.clone(), self.wasi_params.clone())
      }
      None => {
        let mut config = wasmtime::Config::default();
        if self.epoch_deadlines.is_some() {
          config.epoch_interruption(true);
        }

        if self.cache_enabled {
          config.strategy(wasmtime::Strategy::Cranelift);
          if let Some(cache) = &self.cache_path {
            config.cache_config_load(cache).map_err(Error::Initialization)?;
          } else if let Err(e) = config.cache_config_load_default() {
            warn!("Wasmtime cache configuration not found ({}). Repeated loads will speed up significantly with a cache configuration. See https://docs.wasmtime.dev/cli-cache.html for more information.",e);
          }
        }

        #[cfg(feature = "profiler")]
        config.profiler(wasmtime::ProfilingStrategy::JitDump);

        let engine = wasmtime::Engine::new(&config).map_err(Error::Initialization)?;
        WasmtimeEngineProvider::new_with_engine(self.module_bytes, engine, self.wasi_params.clone())
      }
    }?;
    provider.epoch_deadlines = self.epoch_deadlines;

    Ok(provider)
  }
}
