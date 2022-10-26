use std::str::FromStr;

use strum::EnumIter;
pub use strum::IntoEnumIterator;

/// Functions called by guest, exported by host
pub mod host_exports {
  /// The wasmRS protocol function `__init_buffers`
  pub const INIT: &str = "__init_buffers";
  /// The wasmRS protocol function `__send`
  pub const SEND: &str = "__send";
  /// The wasmRS protocol function `__console_log`
  pub const LOG: &str = "__console_log";
  /// The wasmRS protocol function `__op_list`
  pub const OP_LIST: &str = "__op_list";
}

/// The exported host functions as an enum.
#[derive(Debug, Copy, Clone, EnumIter)]
#[allow(missing_docs)]
pub enum HostExports {
  Init,
  Send,
  Log,
  OpList,
}

impl FromStr for HostExports {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let result = match s {
      host_exports::INIT => Self::Init,
      host_exports::SEND => Self::Send,
      host_exports::LOG => Self::Log,
      host_exports::OP_LIST => Self::OpList,
      _ => return Err(()),
    };
    Ok(result)
  }
}

impl AsRef<str> for HostExports {
  fn as_ref(&self) -> &str {
    match self {
      Self::Init => host_exports::INIT,
      Self::Send => host_exports::SEND,
      Self::Log => host_exports::LOG,
      Self::OpList => host_exports::OP_LIST,
    }
  }
}

impl std::fmt::Display for HostExports {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_ref())
  }
}

/// Functions called by host, exported by guest
pub mod guest_exports {
  /// The wasmRS protocol function `__wasmrs_init`
  pub const INIT: &str = "__wasmrs_init";

  /// The wasmRS protocol function `__wasmrs_send`
  pub const SEND: &str = "__wasmrs_send";

  /// The wasmRS protocol function `__wasmrs_op_list_request`
  pub const OP_LIST_REQUEST: &str = "__wasmrs_op_list_request";

  /// The wasmRS protocol function `_start`
  pub const TINYGO_START: &str = "_start";

  /// Start functions to attempt to call - order is important
  pub const REQUIRED_STARTS: [&str; 2] = [TINYGO_START, INIT];
}

/// The exported guest functions as an enum.
#[derive(Debug, Copy, Clone, EnumIter)]
#[allow(missing_docs)]
pub enum GuestExports {
  Init,
  Start,
  OpListRequest,
  Send,
}

impl FromStr for GuestExports {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let result = match s {
      guest_exports::INIT => Self::Init,
      guest_exports::TINYGO_START => Self::Start,
      guest_exports::OP_LIST_REQUEST => Self::OpListRequest,
      guest_exports::SEND => Self::Send,
      _ => return Err(()),
    };
    Ok(result)
  }
}

impl AsRef<str> for GuestExports {
  fn as_ref(&self) -> &str {
    match self {
      GuestExports::Init => guest_exports::INIT,
      GuestExports::Start => guest_exports::TINYGO_START,
      GuestExports::Send => guest_exports::SEND,
      GuestExports::OpListRequest => guest_exports::OP_LIST_REQUEST,
    }
  }
}

impl std::fmt::Display for GuestExports {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.as_ref())
  }
}
