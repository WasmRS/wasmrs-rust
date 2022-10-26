#![cfg_attr(not(feature = "std"), no_std)]
#![warn(clippy::unreadable_literal)]
#![warn(clippy::option_if_let_else)]
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::missing_panics_doc)]

extern crate zerocopy;

pub mod decode;
pub mod encode;
#[cfg(feature = "ext")]
pub mod ext;
mod marker;

#[cfg(feature = "ext")]
pub use ext::*;
#[cfg(feature = "serde_bytes")]
pub use serde_bytes::Bytes;
