---
"@biomejs/biome": minor
---

Added [ignoreRestSiblings](https://github.com/biomejs/biome/issues/5941) option
to `noUnusedFunctionParameters` rule to ignore unused function parameters that
are siblings of the rest parameter. Default is `false`, which means that unused
function parameters that are siblings of the rest parameter will be reported.

## Example

```json
{
  "rules": {
    "noUnusedFunctionParameters": ["error", { "ignoreRestSiblings": true }]
  }
}
```
