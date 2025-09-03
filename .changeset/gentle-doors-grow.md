---
"@biomejs/biome": patch
---

Reduced the severity of certain diagnostics emitted when Biome deserializes the configuration files.
Now these diagnostics are emitted as `Information` severity, which means that they won't interfere when running commands with `--error-on-warnings`
