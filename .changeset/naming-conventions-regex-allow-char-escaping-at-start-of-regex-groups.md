---
"@biomejs/biome": patch
---

The lint rules `useNamingConvention` and `useFilenamingConvention` now accepts character escapes at the start of a regex group.

The rules `useNamingConvention` and `useFilenamingConvention` provides some options that allow matching names against a regular expression.
Previously, an escaped character at the start of a regex group reported an error.
They are now accepted.

For example, the following configuration is now valid doesn't emit an error anymore.

```json
{
  "linter": {
    "rules": {
      "style": {
        "useNamingConvention": {
          "level": "on",
          "options": {
            "conventions": [
              {
                "selector": {
                  "kind": "let"
                },
                "match": "(\\n.*)"
              },
            ]
          }
        }
      }
    }
  }
}
```
