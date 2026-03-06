---
"@biomejs/biome": patch
---

Added the nursery rule [`noTopLevelLiterals`](https://biomejs.dev/linter/rules/no-top-level-literals/). Requiring the root-level value to be an array or object.

**Invalid:**

```json
"just a string"
```
