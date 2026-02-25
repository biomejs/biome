//! Proc macros for the Biome plugin SDK.
//!
//! Provides [`generate_plugin`] which generates WIT bindings without requiring
//! plugin authors to know the filesystem path to the WIT definition.

use proc_macro::TokenStream;

/// The WIT interface definition, embedded at compile time.
const WIT_CONTENT: &str = include_str!("../../biome_plugin_sdk/wit/biome-plugin.wit");

/// Generate WIT bindings for a Biome WASM plugin.
///
/// This expands to `wit_bindgen::generate!` with the Biome plugin WIT
/// definition inlined, so plugin crates do not need to reference the WIT
/// file path directly.
///
/// # Usage
///
/// ```ignore
/// biome_plugin_sdk::generate_plugin!();
/// ```
///
/// The plugin crate must also depend on `wit-bindgen` (for runtime types
/// used by the generated code).
#[proc_macro]
pub fn generate_plugin(_input: TokenStream) -> TokenStream {
    // Produce a `wit_bindgen::generate!` invocation with the WIT content
    // inlined as a raw string literal. We use r####"..."#### to avoid
    // collisions with any content in the WIT file.
    let code = format!(
        r####"::wit_bindgen::generate!({{ inline: r###"{}"###, world: "plugin" }});"####,
        WIT_CONTENT,
    );
    code.parse()
        .expect("failed to parse generated wit_bindgen invocation")
}
