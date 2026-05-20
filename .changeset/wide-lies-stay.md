---
"@biomejs/biome": patch
---

Added the nursery rule [`useConsistentObjectKeys`](https://biomejs.dev/linter/rules/use-consistent-object-keys), which disallows unnormalized JSON object keys.

**Invalid:**

```json
{
  "caf\u0065\u0301": "value"
}
```
