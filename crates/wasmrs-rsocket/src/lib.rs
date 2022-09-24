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
  const_err,
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
  // missing_docs
)]
#![doc = include_str!("../README.md")]
// TODO REMOVE
#![allow(unused, clippy::needless_pass_by_value, unreachable_pub)]

pub mod frames;
pub mod util;

// mod buffers;
mod generated;

use std::io::Read;

pub use frames::FrameCodec;
pub use generated::{ErrorCode, FragmentedPayload, Frame, FrameType, Metadata};

use self::util::from_u32_bytes;

pub fn read_frame(mut buf: impl Read) -> std::io::Result<Vec<u8>> {
    let mut len_bytes = [0u8; 4];
    buf.read_exact(&mut len_bytes)?;
    let len = from_u32_bytes(&len_bytes);
    let mut frame = vec![0; len as usize];
    buf.read_exact(&mut frame)?;
    Ok(frame)
}

// #[no_mangle]
// #[export_name = "init"]
// extern "C" fn init(
//     guest_buffer_position: u32,
//     host_buffer_position: u32,
//     host_max_frame_size: u32,
// ) {
// }

// #[no_mangle]
// #[export_name = "send"]
// extern "C" fn send(next_pos: u32) {}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use wasmrs_ringbuffer::{RingBuffer, VecRingBuffer};

    use crate::read_frame;

    #[test]
    fn test_read_frame() -> Result<()> {
        let mut buf: &[u8] = &[0, 0, 0, 4, 1, 2, 3, 4];
        let frame = read_frame(&mut buf)?;
        assert_eq!(frame, vec![1, 2, 3, 4]);

        let mut rb: VecRingBuffer<u8> = VecRingBuffer::new();
        rb.write_at(0, [0, 0, 0, 4, 1, 2, 3, 4].to_vec());
        println!("{:?}", rb.buffer());
        let frame = read_frame(&mut rb)?;
        assert_eq!(frame, vec![1, 2, 3, 4]);

        Ok(())
    }
}
