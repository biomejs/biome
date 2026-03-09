# WASM Plugin E2E Tests

End-to-end tests for Biome's WASM plugin system.

## Structure

```
wasm-plugins/
  biome.json          # Configuration with plugin paths and options
  demo.js             # JavaScript test file with intentional violations
  demo.css            # CSS test file with intentional violations
  demo.json           # JSON test file with intentional violations
  test.sh             # Test script that builds plugins and verifies output
  plugins/
    boolean-naming/   # JS plugin: checks boolean variable naming conventions
    css-style-conventions/  # CSS plugin: custom property patterns, !important checks
    json-naming/      # JSON plugin: key naming convention enforcement
```

## Running

```sh
cd e2e-tests/wasm-plugins
sh test.sh
```

Prerequisites:
- `wasm32-wasip2` target installed (`rustup target add wasm32-wasip2`)
- Biome built with `wasm_plugin` feature

The test script will:
1. Build all three plugin WASM binaries
2. Run `biome lint` on each demo file
3. Verify diagnostic messages and rule names appear in output

## Plugin Configuration

`biome.json` configures which plugins to load. Each entry is a path to a
compiled `.wasm` file:

```json
{
  "plugins": [
    "plugins/boolean-naming/target/wasm32-wasip2/release/boolean_naming.wasm",
    "plugins/css-style-conventions/target/wasm32-wasip2/release/css_style_conventions.wasm",
    "plugins/json-naming/target/wasm32-wasip2/release/json_naming.wasm"
  ]
}
```

Plugins that accept options can use the object form instead:

```json
{
  "plugins": [
    {
      "path": "plugins/my-plugin.wasm",
      "options": { "convention": "camelCase" }
    }
  ]
}
```

## Adding a New Test Plugin

1. Create a new crate under `plugins/` with `crate-type = ["cdylib"]`
2. Add `biome_plugin_sdk` and `wit-bindgen` as dependencies
3. Implement the `Guest` trait (see existing plugins for examples)
4. Add the build command to `test.sh`
5. Add verification grep checks to `test.sh`
6. Add the plugin path to `biome.json`
