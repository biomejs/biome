# biome_wasm_plugin

Host-side WASM Component Model engine for Biome plugins.

This crate provides `WasmPluginEngine`, which compiles and executes WASM plugins
using `wasmtime`. It handles:

- **Compilation** — compiles WASM bytes into a pre-instantiated component with
  all host imports (syntax tree navigation, semantic model, regex, etc.)
  resolved via `PluginPre`.
- **Metadata extraction** — calls `target-language()`, `rule-names()`,
  `query-kinds-for-rule()`, and `rule-metadata()` exports to discover plugin
  capabilities.
- **Node checking** — creates a fresh `Store<HostState>` per node, optionally
  calls `configure()` with options JSON, then calls `check()` and converts
  results to `RuleDiagnostic` with code actions.

## Host State

`HostState` (in `host_state.rs`) implements the WIT `host` interface, providing
the guest with access to:

- Syntax tree navigation (node kind, text, range, children, parent)
- Semantic model (references, scopes, bindings) — JavaScript only
- Type inference (expression types, literal values) — JavaScript only
- Regex matching utilities
- File path context

## Fuel Budget

Each `check()` call runs with a fuel budget (1,000,000 units) to prevent
infinite loops in guest code. If the budget is exceeded, an out-of-fuel
diagnostic is returned instead of crashing.
