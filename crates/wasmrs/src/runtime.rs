#[cfg(target_family = "wasm")]
pub mod wasm;
#[cfg(target_family = "wasm")]
pub use wasm::*;

#[cfg(not(target_family = "wasm"))]
pub mod native;
#[cfg(not(target_family = "wasm"))]
pub use native::*;
