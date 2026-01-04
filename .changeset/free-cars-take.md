---
"@biomejs/biome": patch
---

Added the nursery rule [`useUniqueEnumValueNames`](https://biomejs.dev/linter/rules/use-unique-enum-value-names/). Enforce unique enum value names.

**Invalid:**

```graphql
enum A {
  TEST
  TesT
}
```
