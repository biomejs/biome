---
"@biomejs/biome": patch
---

Added the nursery rule [`noUnnormalizedObjectKeys`](https://biomejs.dev/linter/rules/no-unnormalized-object-keys), which disallows unnormalized JSON object keys.

**Invalid:**

```json
{
  "caf\u0065\u0301": "value"
}
```
