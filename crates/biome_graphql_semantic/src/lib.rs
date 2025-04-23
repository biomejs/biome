#![deny(clippy::use_self)]

mod events;
mod semantic_model;

pub use events::*;
pub use semantic_model::*;

#[cfg(test)]
mod tests;
