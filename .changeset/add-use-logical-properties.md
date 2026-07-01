---
"@biomejs/biome": patch
---

Added a new nursery rule [`useLogicalProperties`](https://biomejs.dev/linter/rules/use-logical-properties) that enforces the use of logical properties in CSS, promoting better internationalization and accessibility practices.

```json
{
  "linter": {
    "rules": {
      "nursery": {
        "useLogicalProperties": {
          "level": "warn"
        }
      }
    }
  }
}
```
