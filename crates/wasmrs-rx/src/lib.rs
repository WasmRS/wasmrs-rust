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
// TODO REMOVE
#![allow(clippy::needless_pass_by_value, unused)]

mod error;
mod flux;

pub use flux::*;

pub use error::Error;
use futures::Stream;

/// A generic trait to wrap over Flux, Mono, and supporting types.
pub trait FluxLike<I, E>: Stream<Item = Result<I, E>> + Unpin + Send {}

impl<I, E, T> FluxLike<I, E> for T where T: Stream<Item = Result<I, E>> + Unpin + Send {}

#[cfg(test)]
mod test {
  use super::*;
  use anyhow::Result;
  use futures::{Stream, StreamExt};

  #[tokio::test]
  async fn test_basic() -> Result<()> {
    async fn takes_any(mut stream: impl FluxLike<u32, u32>) -> Vec<u32> {
      let mut acc = vec![];
      while let Some(Ok(v)) = stream.next().await {
        acc.push(v);
      }
      acc
    }
    let flux = Flux::<u32, u32>::new();
    flux.send(1)?;
    flux.send(2)?;
    flux.send(3)?;
    flux.send(4)?;
    flux.complete();

    println!("waiting for flux results");
    let results = takes_any(flux).await;
    assert_eq!(results, vec![1, 2, 3, 4]);

    let mono = Mono::<u32, u32>::from_future(async move { Ok(42) });
    println!("waiting for mono results");
    let results = takes_any(mono).await;
    assert_eq!(results, vec![42]);
    Ok(())
  }
}
