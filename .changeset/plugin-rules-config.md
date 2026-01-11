---
"@biomejs/biome": minor
---

Add `pluginRules` configuration option to enable/disable individual plugin rules via `biome.json`. Plugin rules can be configured at the top level or within overrides.

Example configuration:
```json
{
  "linter": {
    "pluginRules": {
      "my-plugin": "off"
    }
  },
  "overrides": [{
    "includes": ["scripts/**"],
    "linter": {
      "pluginRules": { "my-plugin": "off" }
    }
  }]
}
```

Plugin rule names are derived from the Grit pattern filename (e.g., `my-plugin.grit` → `"my-plugin"`).
