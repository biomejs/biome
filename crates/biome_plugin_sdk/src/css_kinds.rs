//! CSS syntax kind constants.
//!
//! These values correspond to the `#[repr(u16)]` discriminants of
//! `biome_css_syntax::CssSyntaxKind`.
//!
//! This file is auto-generated from the canonical `kind.rs` by
//! `biome_plugin_sdk/build.rs`. All variants are included.

include!(concat!(env!("OUT_DIR"), "/css_kinds_generated.rs"));
