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
- `"off"`: Disable the plugin entirely
- `"warn"`: Enable the plugin (uses plugin's own severity)
- `"error"`: Enable the plugin (default)

This allows disabling plugins per-path via overrides:

```json
{
  "plugins": ["./my-plugin.grit"],
  "overrides": [{
    "includes": ["scripts/**"],
    "plugins": [{ "path": "./my-plugin.grit", "severity": "off" }]
  }]
}
```

Note: Severity override (actually changing diagnostics from error to warning) is planned for a future release.
