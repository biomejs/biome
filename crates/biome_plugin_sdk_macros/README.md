# biome_plugin_sdk_macros

Proc macros for the Biome plugin SDK.

## `generate_plugin!()`

Generates WIT bindings for a Biome WASM plugin by expanding to a
`wit_bindgen::generate!` invocation with the WIT definition inlined. This means
plugin crates do not need to know the filesystem path to `biome-plugin.wit`.

### Usage

```rust
biome_plugin_sdk::generate_plugin!();
```

The plugin crate must also depend on `wit-bindgen` (for runtime types used by
the generated code).

### How It Works

The macro embeds the contents of `biome_plugin_sdk/wit/biome-plugin.wit` at
compile time using `include_str!`, then produces a `wit_bindgen::generate!`
call with the `inline` option. This keeps the WIT definition in a single
location while allowing plugin authors to use it without path concerns.
