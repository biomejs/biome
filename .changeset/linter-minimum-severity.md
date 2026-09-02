---
"@biomejs/biome": minor
---

Added the `linter.minimumSeverity` option, which raises the severity of every lint rule that would emit a diagnostic below the given threshold. It accepts `info` (the default, which changes nothing), `warn`, and `error`.

This lets you make sure that lint rules emitting information diagnostics are still reported as warnings or errors, without having to enumerate them one by one:

```json
{
  "linter": {
    "minimumSeverity": "warn"
  }
}
```

Rules configured with a higher severity keep their own severity, and the option can also be set inside `overrides`.

The option only affects diagnostics emitted by lint rules. Diagnostics that don't belong to a rule, such as the ones emitted for an outdated configuration schema or for a file Biome can't parse, keep their severity.
