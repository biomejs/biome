---
"@biomejs/biome": patch
---

Added the nursery rule [`useUniqueInputFieldNames`](https://biomejs.dev/linter/rules/use-unique-input-field-names/). Require fields within an input object to be unique.

**Invalid:**

```graphql
query A($x: Int, $x: Int) {
  field
}
```
