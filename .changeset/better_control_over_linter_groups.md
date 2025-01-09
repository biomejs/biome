---
cli: minor
---

# Better control over linter groups

Linter groups now accept new options to enable/disable all rules that belong to a group, and control the severity
of the rule that belong to those groups.

For example, it's you can downgrade the severity of rule that belong to `"style"` to emit `"info"` diagnostics:

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
