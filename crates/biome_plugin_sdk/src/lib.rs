//! Guest SDK for writing Biome WASM plugins.
//!
//! This crate provides everything needed to build a Biome lint rule as a WASM
//! Component Model module:
//!
//! - **Syntax kind constants** — [`js_kinds`], [`css_kinds`], and [`json_kinds`]
//!   modules contain `u32` constants for every syntax node kind in each language.
//!   Use these in `query_kinds_for_rule()` to tell Biome which nodes your rule
//!   inspects, and in `check()` to identify node types via `node-kind`.
//!
//! - **[`generate_plugin!()`]** — proc macro that generates WIT bindings without
//!   requiring plugin authors to reference the WIT file path. Expands to a
//!   `wit_bindgen::generate!` call with the interface inlined.
//!
//! - **[`options`]** — lightweight JSON parsing helpers (`get_string`,
//!   `get_number`, `get_bool`, `get_string_array`) for reading per-rule
//!   configuration from the JSON string passed to `configure()`.
//!
//! # Supported Languages
//!
//! | Language | Module | `target_language()` return value |
//! |---|---|---|
//! | JavaScript/TypeScript | [`js_kinds`] | `"javascript"` |
//! | CSS | [`css_kinds`] | `"css"` |
//! | JSON | [`json_kinds`] | `"json"` |
//!
//! JavaScript plugins have access to the full semantic model (scopes,
//! references, type inference). CSS and JSON plugins only have syntax tree
//! navigation.
//!
//! # Quick Start
//!
//! ```toml
//! [package]
//! name = "my-plugin"
//! edition = "2021"
//!
//! [lib]
//! crate-type = ["cdylib"]
//!
//! [dependencies]
//! biome_plugin_sdk = "0.0.1"
//! wit-bindgen = "0.39"
//! ```
//!
//! ```ignore
//! use biome_plugin_sdk::js_kinds;
//!
//! biome_plugin_sdk::generate_plugin!();
//!
//! struct MyPlugin;
//!
//! impl Guest for MyPlugin {
//!     fn target_language() -> String { "javascript".into() }
//!     fn rule_names() -> Vec<String> { vec!["myRule".into()] }
//!     fn query_kinds_for_rule(_rule: String) -> Vec<u32> { vec![js_kinds::JS_CALL_EXPRESSION] }
//!     fn configure(_rule: String, _options_json: String) {}
//!     fn rule_metadata(_rule: String) -> RuleMetadata { todo!() }
//!     fn check(node: u32, _rule: String) -> Vec<CheckResult> { vec![] }
//! }
//!
//! export!(MyPlugin);
//! ```
//!
//! Build with: `cargo build --target wasm32-wasip2 --release`
//!
//! # Examples
//!
//! See `e2e-tests/wasm-plugins/plugins/` for working examples covering
//! JavaScript, CSS, and JSON plugins.

pub use biome_plugin_sdk_macros::generate_plugin;

pub mod css_kinds;
pub mod js_kinds;
pub mod json_kinds;
pub mod options;
