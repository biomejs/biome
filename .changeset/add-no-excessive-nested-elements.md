---
"@biomejs/biome": patch
---

Added a new nursery rule [`noExcessiveNestedElements`](https://biomejs.dev/linter/rules/no-excessive-nested-elements/) that enforces a maximum nesting depth for JSX elements (default: 10). Use the `maxDepth` option to configure the limit.

```json
{
  "linter": {
    "rules": {
      "nursery": {
        "noExcessiveNestedElements": {
          "level": "warn",
          "options": {
            "maxDepth": 5
          }
        }
      }
    }
  }
}
```
