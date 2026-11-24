#![deny(clippy::use_self)]

mod capabilities;
mod diagnostics;
mod documents;
mod extension_settings;
mod handlers;
mod requests;
mod server;
#[cfg(test)]
#[path = "server_svelte.tests.rs"]
mod server_svelte;
#[cfg(test)]
mod server_test_utils;
mod session;
mod utils;

pub use crate::extension_settings::WorkspaceSettings;
pub use crate::server::{LSPServer, ServerConnection, ServerFactory};
