# biome_plugin_sdk

Guest SDK for writing Biome WASM plugins.

This crate provides everything a plugin author needs to build a Biome lint rule
as a WASM Component Model module:

- **Syntax kind constants** for JavaScript (`js_kinds`), CSS (`css_kinds`), and
  JSON (`json_kinds`) so plugins can match specific AST node types.
- **`generate_plugin!()`** macro that generates WIT bindings without requiring
  the plugin to reference the WIT file path.
- **`options`** module with lightweight JSON parsing helpers for reading
  per-rule configuration without pulling in a full JSON library.

## Quick Start

```toml
[package]
name = "my-plugin"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
biome_plugin_sdk = "0.0.1"
wit-bindgen = "0.39"
```

```rust
use biome_plugin_sdk::js_kinds;

biome_plugin_sdk::generate_plugin!();

struct MyPlugin;

impl Guest for MyPlugin {
    fn target_language() -> String { "javascript".into() }
    fn rule_names() -> Vec<String> { vec!["myRule".into()] }
    fn query_kinds_for_rule(_rule: String) -> Vec<u32> {
        vec![js_kinds::JS_CALL_EXPRESSION]
    }
    fn configure(_rule: String, _options_json: String) {}
    fn rule_metadata(_rule: String) -> RuleMetadata {
        RuleMetadata {
            version: "0.1.0".into(),
            sources: vec![],
            recommended: true,
            fix_kind: None,
            category: None,
            domains: vec![],
            deprecated: None,
            severity: None,
            issue_number: None,
        }
    }
    fn check(node: u32, _rule: String) -> Vec<CheckResult> {
        vec![]
    }
}

export!(MyPlugin);
```

Build with:

```sh
cargo build --target wasm32-wasip2 --release
```

## Supported Languages

| Language | Module | Target string | Semantic model |
|---|---|---|---|
| JavaScript/TypeScript | `js_kinds` | `"javascript"` | Full (scopes, references, types) |
| CSS | `css_kinds` | `"css"` | None |
| JSON | `json_kinds` | `"json"` | None |

## Options

The `options` module provides `get_string`, `get_number`, `get_bool`, and
`get_string_array` functions for parsing the JSON options string passed to
`configure()`. These avoid pulling in `serde_json` in the WASM binary.

## WIT Interface

The full plugin interface is defined in `wit/biome-plugin.wit`. Key exports
that a plugin must implement:

- `target-language()` — which language this plugin analyzes
- `rule-names()` — list of rule names (used in suppressions and config)
- `query-kinds-for-rule(rule)` — syntax node kinds to match
- `configure(rule, options-json)` — receive per-rule options
- `check(node, rule)` — analyze a matched node and return diagnostics
- `rule-metadata(rule)` — version, sources, fix kind, etc.

## Examples

See `e2e-tests/wasm-plugins/plugins/` for working examples:

- `boolean-naming/` — JavaScript rule checking boolean variable naming
- `css-style-conventions/` — CSS rule checking custom property patterns
- `json-naming/` — JSON rule checking key naming conventions
