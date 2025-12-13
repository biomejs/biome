---
"@biomejs/biome": patch
---

Fixed [#8429](https://github.com/biomejs/biome/issues/8429). Formatter, linter, and assist settings now correctly inherit from global configuration when not explicitly specified in overrides.

Before this fix, when an override specified only one feature (e.g., only `linter`), other features would be incorrectly disabled instead of inheriting from global settings.

Example configuration that now works correctly:
```json
{
  "formatter": { "enabled": true },
  "overrides": [{
    "includes": ["*.vue"],
    "linter": { "enabled": false }
  }]
}
```

After this fix, `.vue` files will have the linter disabled (as specified in the override) but the formatter enabled (inherited from global settings).
