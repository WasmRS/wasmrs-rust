use std::path::PathBuf;

/// Parameters defining the options for enabling WASI on a module (if applicable)
#[derive(Debug, Default, Clone, Eq, PartialEq)]
#[must_use]
pub struct WasiParams {
  /// Command line arguments to expose to WASI.
  pub argv: Vec<String>,
  /// A mapping of directories.
  pub map_dirs: Vec<(String, PathBuf)>,
  /// Environment variables and values to expose.
  pub env_vars: Vec<(String, String)>,
  /// Directories that WASI has access to.
  pub preopened_dirs: Vec<PathBuf>,
}

impl WasiParams {
  /// Instantiate a new WasiParams struct.
  pub fn new(
    argv: Vec<String>,
    map_dirs: Vec<(String, PathBuf)>,
    env_vars: Vec<(String, String)>,
    preopened_dirs: Vec<PathBuf>,
  ) -> Self {
    WasiParams {
      argv,
      map_dirs,
      preopened_dirs,
      env_vars,
    }
  }
}
