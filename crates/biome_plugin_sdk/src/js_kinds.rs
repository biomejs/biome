//! JavaScript/TypeScript syntax kind constants.
//!
//! These values correspond to the `#[repr(u16)]` discriminants of
//! `biome_js_syntax::JsSyntaxKind`. They are used in `query_kinds()`
//! to tell the host which node kinds the plugin wants to inspect.
//!
//! This file is auto-generated from the canonical `kind.rs` by
//! `biome_plugin_sdk/build.rs`. All variants are included.

include!(concat!(env!("OUT_DIR"), "/js_kinds_generated.rs"));
