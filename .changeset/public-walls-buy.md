---
"@biomejs/biome": patch
---

Added the nursery rule [`useUniqueVariableNames`](https://biomejs.dev/linter/rules/use-unique-variable-names/). Enforce unique variable names for GraphQL operations.

**Invalid:**

```graphql
query ($x: Int, $x: Int) {
  field
}
```
