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
#![allow(clippy::needless_pass_by_value)]

mod guest;
pub use futures_util::FutureExt;
pub use guest::*;
pub use wasmrs_runtime as runtime;
pub use wasmrs_rx::*;

mod exports;
mod imports;
mod server;

/// The wasmRS-guest error module.
pub mod error;

pub use futures_util::Stream;
pub use serde_json::Value;
// pub use wasmrs_codec::messagepack::Timestamp;

/// Deserialize a generic [Value] from MessagePack bytes.
pub fn deserialize_generic(buf: &[u8]) -> Result<std::collections::BTreeMap<String, Value>, Error> {
  deserialize(buf).map_err(|e| Error::Decode(e.to_string()))
}

cfg_if::cfg_if!(
  if #[cfg(all(feature = "logging", target_os = "wasi"))] {
    /// Turn on logging for the guest (WASI only).
    pub fn init_logging() {
      env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .parse_env("wasmrs")
        .init();
    }
  } else {
    /// Turn on logging for the guest (WASI only).
    pub fn init_logging() {}
  }
);

#[cfg(test)]
mod test {

  use super::*;
  use anyhow::Result;
  #[test]
  fn test_basic() -> Result<()> {
    #[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
    struct Input {
      input: String,
      num: u32,
    }
    let input = Input {
      input: "HELLO WORLD".to_owned(),
      num: 32,
    };
    let bytes = serialize(&input)?;
    let input2: Input = deserialize(&bytes)?;
    assert_eq!(input.input, input2.input);
    assert_eq!(input.num, input2.num);
    println!("{:?}", bytes);
    let map: Value = deserialize(&bytes)?;
    println!("{:?}", map);
    if let Value::Object(map) = map {
      assert_eq!(map.get("input"), Some(&Value::String("HELLO WORLD".to_owned())));
    } else {
      panic!("expected map");
    }

    Ok(())
  }
}
