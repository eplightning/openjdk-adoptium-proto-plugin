#[cfg(feature = "wasm")]
mod proto;

#[cfg(feature = "wasm")]
pub mod adoptium_api;

#[cfg(feature = "wasm")]
pub use proto::*;
