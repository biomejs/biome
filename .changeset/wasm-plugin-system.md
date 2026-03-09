---
"@biomejs/biome": minor
---

Added an opt-in WASM Component Model plugin system behind the `wasm_plugin` feature flag. Users can write custom lint rules in Rust (or any language targeting `wasm32-wasip2`), compile them to `.wasm` modules, and load them at runtime via `wasmtime`. Plugins support full AST traversal, semantic model access, type inference, configurable options, and suppression comments. This complements the existing GritQL plugin system with a high-performance alternative for complex rules that need stateful analysis.
