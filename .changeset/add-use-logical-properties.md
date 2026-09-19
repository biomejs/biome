---
"@biomejs/biome": patch
---

Added a new nursery rule [`useLogicalProperties`](https://biomejs.dev/linter/rules/use-logical-properties) that enforces the use of logical properties in CSS, promoting better internationalization and accessibility practices. The rule supports a `direction` option with `"ltr"` as the default and `"rtl"` as the alternative.

```json
{
  "linter": {
    "rules": {
      "nursery": {
        "useLogicalProperties": {
          "level": "warn",
          "options": {
            "direction": "rtl"
          }
        }
      }
    }
  }
}
```
