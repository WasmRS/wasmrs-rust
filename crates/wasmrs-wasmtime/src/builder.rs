use parking_lot::Mutex;
use wasi_common::WasiCtx;
use wasmtime::Module;

use crate::engine_provider::EpochDeadlines;
use crate::errors::Error;
use crate::wasi::init_wasi;
use crate::WasmtimeEngineProvider;

static MODULE_CACHE: once_cell::sync::Lazy<Mutex<std::collections::HashMap<String, Module>>> =
  once_cell::sync::Lazy::new(|| Mutex::new(std::collections::HashMap::new()));

/// Used to build [`WasmtimeEngineProvider`](crate::WasmtimeEngineProvider) instances.
#[allow(missing_debug_implementations)]
#[must_use]
#[derive(Default)]
pub struct WasmtimeBuilder<'a> {
  engine: Option<wasmtime::Engine>,
  module: Option<Module>,
  module_bytes: Option<(String, &'a [u8])>,
  cache_enabled: bool,
  cache_path: Option<std::path::PathBuf>,
  wasi_params: Option<wasmrs_host::WasiParams>,
  wasi_ctx: Option<WasiCtx>,
  epoch_deadlines: Option<EpochDeadlines>,
}

impl<'a> WasmtimeBuilder<'a> {
  /// A new [WasmtimeBuilder] instance.
  pub fn new() -> Self {
    WasmtimeBuilder { ..Default::default() }
  }

  /// Query if the module cache contains a module with the given id.
  pub fn is_cached(id: impl AsRef<str>) -> bool {
    let lock = MODULE_CACHE.lock();
    lock.contains_key(id.as_ref())
  }

  /// Initialize the builder with a preloaded module.
  pub fn with_cached_module(mut self, id: impl AsRef<str>) -> Result<Self, Error> {
    let lock = MODULE_CACHE.lock();
    if let Some(module) = lock.get(id.as_ref()) {
      self.module = Some(module.clone());
      return Ok(self);
    }
    Err(Error::NotFound(id.as_ref().to_owned()))
  }

  /// Initialize the builder with a module from the passed bytes, caching it with the passed ID.
  pub fn with_module_bytes(mut self, id: impl AsRef<str>, bytes: &'a [u8]) -> Self {
    self.module_bytes = Some((id.as_ref().to_owned(), bytes));
    self
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

  /// WASI params, for basic WASI support.
  ///
  /// **Warning:** this has no effect when a custom [`WasiCtx`] is provided via the
  /// [`WasmtimeEngineProviderBuilder::wasi_ctx`] helper.
  pub fn wasi_params(mut self, wasi: wasmrs_host::WasiParams) -> Self {
    self.wasi_params = Some(wasi);
    self
  }

  /// Wasmtime WASI Context, for when you need more control over the WASI environment.
  pub fn wasi_ctx(mut self, wasi: WasiCtx) -> Self {
    self.wasi_ctx = Some(wasi);
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
  pub fn build(self) -> Result<WasmtimeEngineProvider, Error> {
    let engine = match &self.engine {
      Some(e) => {
        // note: we have to call `.clone()` because `e` is behind
        // a shared reference and `Engine` does not implement `Copy`.
        // However, cloning an `Engine` is a cheap operation because
        // under the hood wasmtime does not create a new `Engine`, but
        // rather creates a new reference to it.
        // See https://docs.rs/wasmtime/latest/wasmtime/struct.Engine.html#engines-and-clone
        e.clone()
      }
      None => {
        let mut config = wasmtime::Config::default();
        config.async_support(true);

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

        wasmtime::Engine::new(&config).map_err(Error::Initialization)?
      }
    };

    let module = match (self.module, self.module_bytes) {
      (Some(m), None) => m.clone(),
      (None, Some((id, bytes))) => {
        let module = Module::from_binary(&engine, bytes).map_err(Error::Initialization)?;
        let mut lock = MODULE_CACHE.lock();
        lock.insert(id.clone(), module.clone());
        module
      }
      (None, None) => return Err(Error::NoModule),
      _ => return Err(Error::AmbiguousModule),
    };

    let epoch_deadlines = self.epoch_deadlines;

    let ctx = if self.wasi_ctx.is_some() {
      self.wasi_ctx
    } else if let Some(wasi_params) = self.wasi_params {
      Some(init_wasi(&wasi_params)?)
    } else {
      None
    };

    let mut provider = WasmtimeEngineProvider::new_with_engine(module, engine, ctx)?;

    provider.epoch_deadlines = epoch_deadlines;

    Ok(provider)
  }
}
