---
"@biomejs/biome": patch
---

Added the nursery rule [`useUniqueArgumentNames`](https://biomejs.dev/linter/rules/use-unique-argument-names/). Enforce unique arguments for GraphQL fields & directives.

**Invalid:**

```graphql
query {
  field(arg1: "value", arg1: "value")
}
```
