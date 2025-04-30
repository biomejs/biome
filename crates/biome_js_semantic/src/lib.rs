#![deny(clippy::use_self)]

mod events;
mod format_semantic_model;

mod semantic_model;
#[cfg(test)]
mod tests;

pub use events::*;
pub use semantic_model::*;
