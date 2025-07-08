---
"@biomejs/biome": minor
---

Added the `ignoreRestSiblings` option to the `noUnusedFunctionParameters` rule.

This option is used to ignore unused function parameters that are siblings of the rest parameter.

The default is `false`, which means that unused function parameters that are siblings of the rest parameter will be reported.

**Example**

```json
{
  "rules": {
    "noUnusedFunctionParameters": ["error", { "ignoreRestSiblings": true }]
  }
}
```
