//! A semantic model for Vue components.
//!
//! Vue spreads one component over several languages and many syntax forms. A
//! prop can be declared in an options object, in a `defineProps()` call, or
//! in a TypeScript type, and it is used from a template written in HTML. This
//! crate reads all of those and describes the result in one model, so that
//! lint rules can ask about props, emits or template variables without
//! knowing how each was written.
//!
//! The model has two layers:
//!
//! - The component layer works on one JavaScript tree. It finds the
//!   components the tree defines and records their declarations. It is all a
//!   JavaScript or TypeScript file needs. See [`component_model`].
//! - The single-file component layer works on a `.vue` file. It adds the
//!   template and resolves the names the template and the scripts share. See
//!   [`sfc_model`].

#![deny(clippy::use_self)]

mod format_semantic_model;
mod semantic_model;
#[cfg(test)]
mod tests;

pub use semantic_model::*;
