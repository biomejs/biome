---
"@biomejs/biome": minor
---

Added a new linter configuration called `preset`. With the new option, users can enable different kinds of rules at once.

The following presets are available:
- `"recommended"`: it enables all Biome-recommended rules, or recommended rules of a group;
- `"all"`: it enables all Biome rules, or enables all rules of a group;
- `"none"`: it disables all Biome rules, or disable all rules of a group.

You can enable recommended rules:

```json
{
  "linter": {
    "rules": {
      "preset": "recommended"
    }
  }
}
```

You can enable **all rules** at once:

```json5
{
  "linter": {
    "rules": {
      "preset": "all" // enables all rules
    }
  }
}
```

Or enable all rules for a group:
```json5
{
  "linter": {
    "rules": {
      "style": {
        "preset": "all" // enables all rules in the style group
      },
    }
  }
}
```

This new option, however, doesn't affect how nursery rules work. Nursery rules must be enabled singularly, due to their nature.

This new option is meant to replace `recommended`, so make sure to run the `migrate` command.


