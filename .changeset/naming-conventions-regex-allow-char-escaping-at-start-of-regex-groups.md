---
"@biomejs/biome": patch
---

The lint rules [`useNamingConvention`](https://biomejs.dev/linter/rules/use-naming-convention/) and [`useFilenamingConvention`](https://biomejs.dev/linter/rules/use-filenaming-convention/) now accept character escapes at the start of a regex group.

Both these rules provide options that allow matching names against a regular expression.
Previously, an escaped character at the start of a regex group reported an error. They are now accepted.

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
