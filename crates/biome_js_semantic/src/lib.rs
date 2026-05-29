#![deny(clippy::use_self)]

mod events;
mod format_semantic_model;

mod db;
mod semantic_model;
#[cfg(test)]
mod tests;

pub use db::{JsSemanticDb, js_semantic_model};
pub use events::*;
pub use semantic_model::*;
