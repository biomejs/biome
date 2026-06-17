#![deny(clippy::use_self)]

mod events;
mod format_semantic_model;

mod db;
mod semantic_model;
#[cfg(test)]
mod tests;

pub use db::{
    JsSemanticDb, js_semantic_model, semantic_model_from_snippet, semantic_model_from_source,
};
pub use events::*;
pub use semantic_model::*;
