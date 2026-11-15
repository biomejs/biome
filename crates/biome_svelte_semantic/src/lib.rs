//! A semantic model for Svelte components.
//!
//! A Svelte component spreads one program over several languages. A variable
//! declared in the instance script is used by the template, which is written
//! in HTML with JavaScript expressions, and a variable declared by the
//! template, such as the item of `{#each}`, is only visible to part of it.
//! This crate reads the scripts and the template of a component and
//! describes the result in one model, so that lint rules can ask what a
//! name refers to, which runes initialize a variable, or which props a
//! component declares, without knowing how each was written.
//!
//! The model follows the semantics of runes mode. It also reports whether
//! Svelte compiles the component in runes mode at all, so that rules can
//! skip components written for the legacy mode.
//!
//! Build the model of a component with [`component_model`], and that of a
//! `.svelte.js` module with [`module_model`].

#![deny(clippy::use_self)]

mod format_semantic_model;
mod semantic_model;
#[cfg(test)]
mod tests;

pub use semantic_model::*;
