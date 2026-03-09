# biome_plugin_loader

Host-side plugin loading, caching, and evaluation for Biome's analyzer.

This crate is responsible for:

- **Loading** plugin files from disk (WASM, GritQL, or JavaScript)
- **Caching** compiled plugins to avoid repeated recompilation
- **Evaluating** plugins against syntax nodes during analysis
- **Configuration** parsing for the `plugins` array in `biome.json`

## Plugin Types

| Type | File extension | Engine |
| --- | --- | --- |
| WASM | `.wasm` | `wasmtime` (Component Model) via `biome_wasm_plugin` |
| GritQL | `.grit` | `biome_grit_patterns` |
| JavaScript | `.js` / `.mjs` | `boa_engine` |

## Configuration

Plugins are configured in the `plugins` array of `biome.json`. Each entry can
be a simple path string or an object with options:

```json
{
  "plugins": [
    "./plugins/my-rule.wasm",
    {
      "path": "./plugins/configurable.wasm",
      "options": { "convention": "camelCase" },
      "rules": {
        "myRule": "warn"
      }
    }
  ]
}
```

The `options` object is passed as a JSON string to the plugin's `configure()`
export. The `rules` map allows per-rule configuration using severity levels
(`off`, `on`, `info`, `warn`, `error`) or an object with `level` and `fix`.

## Architecture

The `AnalyzerPlugin` trait (defined in `biome_analyze`) is the common interface
implemented by all plugin types. Each plugin type has its own loader
(`AnalyzerWasmPlugin`, `AnalyzerGritPlugin`, `AnalyzerJsPlugin`) that handles
the specifics of compilation and evaluation.

`PluginCache` stores loaded plugins keyed by path and is shared across files in
a workspace session.
