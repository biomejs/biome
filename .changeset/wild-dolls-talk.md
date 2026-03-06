---
"@biomejs/biome": patch
---

Added the nursery rule [`noEmptyObjectKeys`](https://biomejs.dev/linter/rules/no-empty-object-keys/), which disallows the use of empty keys in JSON objects.

**Invalid:**

```json
{
  "": "value"
}
```
