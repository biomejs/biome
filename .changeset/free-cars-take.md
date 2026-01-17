---
"@biomejs/biome": patch
---

Added the nursery rule [`noDuplicateEnumValueNames`](https://biomejs.dev/linter/rules/no-duplicate-enum-value-names/). Enforce unique enum value names.

**Invalid:**

```graphql
enum A {
  TEST
  TesT
}
```
