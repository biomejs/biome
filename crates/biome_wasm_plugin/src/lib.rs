#![deny(clippy::use_self)]

mod engine;
mod host_state;

pub use engine::{WasmPluginEngine, WasmPluginMetadata, WasmPluginSession};
