---
"@biomejs/biome": patch
---

Fixed [#7943](https://github.com/biomejs/biome/issues/7943). Rules' `options` are now properly merged with the inherited `options` from a shared configuration.

This means that you can now override a specific option from a rule without resetting the other options to their default.

Given the following shared configuration:

```json
{
  "linter": {
    "rules": {
      "style": {
        "useNamingConvention": {
          "level": "on",
          "options": {
            "strictCase": false,
            "conventions": [{
              "selector": { "kind": "variable", "scope": "global" },
              "formats": ["CONSTANT_CASE"]
            }]
          }
        }
      }
    }
  }
}
```

And the user configuration that extends this shared configuration:

```json
{
  "extends": ["shared.json"],
  "linter": {
    "rules": {
      "style": {
        "useNamingConvention": {
          "level": "on",
          "options": { "strictCase": true }
        }
      }
    }
  }
}
```

The obtained merged configuration is now as follows:

```json
{
  "extends": ["shared.json"],
  "linter": {
    "rules": {
      "style": {
        "useNamingConvention": {
          "level": "on",
          "options": {
            "strictCase": true,
            "conventions": [{
              "selector": { "kind": "variable", "scope": "global" },
              "formats": ["CONSTANT_CASE"]
            }]
          }
        }
      }
    }
  }
}
```
