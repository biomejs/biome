---
"@biomejs/biome": minor
---

Added `includes` option for plugin file scoping. Plugins can now be configured with glob patterns to restrict which files they run on. Use negated globs for exclusions.

```json
{
  "plugins": [
    "global-plugin.grit",
    { "path": "scoped-plugin.grit", "includes": ["src/**/*.ts", "!**/*.test.ts"] }
  ]
}
```
