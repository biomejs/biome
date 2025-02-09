---
"@biomejs/biome": minor
---

Linter groups now accept new options to enable/disable all rules that belong to a group, and control the severity
of the rules that belong to those groups.

For example, you can downgrade the severity of rules that belong to `"style"` to emit `"info"` diagnostics:

```json
{
  "linter": {
    "rules": {
      "style": "info"
    }
  }
}
```

You can also enable all rules that belong to a group using the default severity of the rule using the `"on"` option:

```json
{
  "linter": {
    "rules": {
      "complexity": "on"
    }
  }
}
```
