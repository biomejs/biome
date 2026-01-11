---
"@biomejs/biome": minor
---

Add `severity` option to plugin configuration. Plugins can now be configured using an object form with `path` and `severity` fields.

```json
{
  "plugins": [
    "./my-plugin.grit",
    { "path": "./other-plugin.grit", "severity": "off" }
  ]
}
```

Supported severity values:
- `"off"`: Disable the plugin entirely (no diagnostics emitted).
- `"warn"`: Override plugin diagnostics to warning severity.
- `"error"`: Override plugin diagnostics to error severity (default).

This allows configuring plugin behavior per-path via overrides:

```json
{
  "plugins": ["./my-plugin.grit"],
  "overrides": [{
    "includes": ["scripts/**"],
    "plugins": [{ "path": "./my-plugin.grit", "severity": "off" }]
  }]
}
```
