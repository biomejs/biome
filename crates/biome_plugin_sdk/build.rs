//! Build script for `biome_plugin_sdk`.
//!
//! Parses the generated `kind.rs` files from `biome_js_syntax`,
//! `biome_css_syntax`, and `biome_json_syntax`, extracts all enum variant
//! names and their implicit `#[repr(u16)]` discriminants, and emits
//! `pub const NAME: u32 = N;` constants into the OUT_DIR.
//!
//! This replaces the hand-maintained curated subsets with auto-generated
//! complete constant files that stay in sync automatically.

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    generate_kinds(
        &manifest_dir.join("../biome_js_syntax/src/generated/kind.rs"),
        "JsSyntaxKind",
        &out_dir.join("js_kinds_generated.rs"),
    );
    generate_kinds(
        &manifest_dir.join("../biome_css_syntax/src/generated/kind.rs"),
        "CssSyntaxKind",
        &out_dir.join("css_kinds_generated.rs"),
    );
    generate_kinds(
        &manifest_dir.join("../biome_json_syntax/src/generated/kind.rs"),
        "JsonSyntaxKind",
        &out_dir.join("json_kinds_generated.rs"),
    );

    // Re-run when source kind files change.
    println!("cargo:rerun-if-changed=../biome_js_syntax/src/generated/kind.rs");
    println!("cargo:rerun-if-changed=../biome_css_syntax/src/generated/kind.rs");
    println!("cargo:rerun-if-changed=../biome_json_syntax/src/generated/kind.rs");
}

/// Parse a `kind.rs` file and emit `pub const` definitions.
fn generate_kinds(source_path: &Path, enum_name: &str, output_path: &Path) {
    let source = fs::read_to_string(source_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", source_path.display()));

    let variants = parse_enum_variants(&source, enum_name);
    let mut output = String::new();
    writeln!(
        output,
        "// Generated file, do not edit by hand, see `biome_plugin_sdk/build.rs`"
    )
    .unwrap();
    writeln!(output).unwrap();

    for (name, discriminant) in &variants {
        writeln!(output, "pub const {name}: u32 = {discriminant};").unwrap();
    }

    fs::write(output_path, output).unwrap();
}

/// Parse `pub enum <enum_name> { ... }` and extract variant names with their
/// implicit `#[repr(u16)]` discriminant values. Skips `#[doc(hidden)]`
/// variants (TOMBSTONE, `__LAST`).
fn parse_enum_variants(source: &str, enum_name: &str) -> Vec<(String, u32)> {
    let mut variants = Vec::new();

    // Find the start of the enum body.
    let enum_header = format!("pub enum {enum_name}");
    let Some(header_pos) = source.find(&enum_header) else {
        panic!("could not find `{enum_header}` in source");
    };

    let after_header = &source[header_pos..];
    let Some(brace_pos) = after_header.find('{') else {
        panic!("could not find opening brace for `{enum_header}`");
    };

    let body_start = header_pos + brace_pos + 1;

    // Find the matching closing brace. We need to handle nested braces
    // from doc comments and attributes.
    let body = &source[body_start..];
    let mut depth = 1u32;
    let mut end_pos = 0;
    for (i, ch) in body.char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    end_pos = i;
                    break;
                }
            }
            _ => {}
        }
    }

    let enum_body = &body[..end_pos];

    let mut discriminant: u32 = 0;
    let mut skip_next = false;

    for line in enum_body.lines() {
        let trimmed = line.trim();

        // Detect `#[doc(hidden)]` — skip the next variant.
        if trimmed == "#[doc(hidden)]" {
            skip_next = true;
            continue;
        }

        // Skip doc comments and other attributes.
        if trimmed.starts_with("#[") || trimmed.starts_with("///") || trimmed.starts_with("//") {
            continue;
        }

        // Skip empty lines.
        if trimmed.is_empty() {
            continue;
        }

        // Extract variant name. Lines look like:
        //   VARIANT_NAME,
        //   VARIANT_NAME = 42,
        let variant_name = trimmed.split([',', ' ', '=']).next().unwrap_or("").trim();

        if variant_name.is_empty() {
            continue;
        }

        // Check for explicit discriminant assignment: `VARIANT = N,`
        if let Some(eq_rest) = trimmed.strip_prefix(variant_name) {
            let eq_rest = eq_rest.trim();
            if let Some(after_eq) = eq_rest.strip_prefix('=') {
                let val_str = after_eq.trim().trim_end_matches(',').trim();
                if let Ok(val) = val_str.parse::<u32>() {
                    discriminant = val;
                }
            }
        }

        if skip_next {
            skip_next = false;
            discriminant += 1;
            continue;
        }

        // Skip variants that start with `__` (internal markers).
        if variant_name.starts_with("__") {
            discriminant += 1;
            continue;
        }

        variants.push((variant_name.to_string(), discriminant));
        discriminant += 1;
    }

    variants
}
