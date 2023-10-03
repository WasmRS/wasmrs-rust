#![deny(
  clippy::expect_used,
  clippy::explicit_deref_methods,
  clippy::option_if_let_else,
  clippy::await_holding_lock,
  clippy::cloned_instead_of_copied,
  clippy::explicit_into_iter_loop,
  clippy::flat_map_option,
  clippy::fn_params_excessive_bools,
  clippy::implicit_clone,
  clippy::inefficient_to_string,
  clippy::large_types_passed_by_value,
  clippy::manual_ok_or,
  clippy::map_flatten,
  clippy::map_unwrap_or,
  clippy::must_use_candidate,
  clippy::needless_for_each,
  clippy::needless_pass_by_value,
  clippy::option_option,
  clippy::redundant_else,
  clippy::semicolon_if_nothing_returned,
  clippy::too_many_lines,
  clippy::trivially_copy_pass_by_ref,
  clippy::unnested_or_patterns,
  clippy::future_not_send,
  clippy::useless_let_if_seq,
  clippy::str_to_string,
  clippy::inherent_to_string,
  clippy::let_and_return,
  clippy::string_to_string,
  clippy::try_err,
  clippy::unused_async,
  clippy::missing_enforced_import_renames,
  clippy::nonstandard_macro_braces,
  clippy::rc_mutex,
  clippy::unwrap_or_else_default,
  clippy::manual_split_once,
  clippy::derivable_impls,
  clippy::needless_option_as_deref,
  clippy::iter_not_returning_iterator,
  clippy::same_name_method,
  clippy::manual_assert,
  clippy::non_send_fields_in_send_ty,
  clippy::equatable_if_let,
  bad_style,
  clashing_extern_declarations,
  dead_code,
  deprecated,
  explicit_outlives_requirements,
  improper_ctypes,
  invalid_value,
  missing_copy_implementations,
  missing_debug_implementations,
  mutable_transmutes,
  no_mangle_generic_items,
  non_shorthand_field_patterns,
  overflowing_literals,
  path_statements,
  patterns_in_fns_without_body,
  private_in_public,
  trivial_bounds,
  trivial_casts,
  trivial_numeric_casts,
  type_alias_bounds,
  unconditional_recursion,
  unreachable_pub,
  unsafe_code,
  unstable_features,
  unused,
  unused_allocation,
  unused_comparisons,
  unused_import_braces,
  unused_parens,
  unused_qualifications,
  while_true,
  missing_docs
)]
#![doc = include_str!("../README.md")]

mod error;
mod handlers;
mod operations;
mod socket;
/// Utility functions related to frames.
pub mod util;
use futures::Stream;
pub use handlers::*;

#[macro_use]
extern crate tracing;

pub use error::Error;
pub use operations::{Operation, OperationList, OperationType};
pub use socket::{BufferState, SocketSide, WasmSocket};
pub use wasmrs_frames::{ErrorCode, Frame, Metadata, RawPayload};

#[cfg(feature = "record-frames")]
mod record;
#[cfg(feature = "record-frames")]
pub use record::{get_records, FrameRecord, FRAME_RECORDS};
use wasmrs_runtime::{ConditionallySend, ConditionallySendSync};
pub use wasmrs_rx::Flux;

pub use wasmrs_rx::{BoxFlux, BoxMono};

type Result<T> = std::result::Result<T, Error>;

pub use wasmrs_frames::PayloadError;

/// A trait that defines the interface for a wasmRS module host.
pub trait ModuleHost: Sync + Send {
  /// Write a frame to a wasmRS module's memory buffer.
  fn write_frame(&mut self, frame: Frame) -> Result<()>;

  /// Get an imported operation's index.
  fn get_export(&self, namespace: &str, operation: &str) -> Result<u32>;

  /// Get an exported operation's index.
  fn get_import(&self, namespace: &str, operation: &str) -> Result<u32>;

  /// Get a cloned operation list.
  fn get_operation_list(&self) -> OperationList;
}

/// A trait for an RSocket client/server (host/guest).
pub trait RSocket: ConditionallySendSync {
  /// Fire and Forget interaction model of RSocket.
  fn fire_and_forget(&self, payload: RawPayload) -> BoxMono<(), PayloadError>;
  /// Request-Response interaction model of RSocket.
  fn request_response(&self, payload: RawPayload) -> BoxMono<RawPayload, PayloadError>;
  /// Request-Stream interaction model of RSocket.
  fn request_stream(&self, payload: RawPayload) -> BoxFlux<RawPayload, PayloadError>;
  /// Request-Channel interaction model of RSocket.
  fn request_channel<
    T: Stream<Item = std::result::Result<RawPayload, PayloadError>> + ConditionallySend + Unpin + 'static,
  >(
    &self,
    stream: T,
  ) -> BoxFlux<RawPayload, PayloadError>;
}
