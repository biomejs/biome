//! A semantic model for Astro documents.
//!
//! Astro spreads one document over server-side frontmatter, template
//! expressions, and isolated client scripts. This crate records those scopes
//! and their declarations and references so analyzers can query Astro semantics
//! without reproducing the framework rules themselves.

#![deny(clippy::use_self)]

mod format_semantic_model;
mod semantic_model;
#[cfg(test)]
mod tests;

pub use semantic_model::*;
