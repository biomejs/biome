---
"@biomejs/biome": patch
---

Added the nursery rule [`useConsistentGraphqlDescriptions`](https://biomejs.dev/linter/rules/use-consistent-graphql-descriptions/), requiring all descriptions to follow the same style (either block or inline) inside GraphQL files.

**Invalid example:**

```graphql
enum EnumValue {
  "this is a description"
  DEFAULT
}
```

**Valid example:**

```graphql
enum EnumValue {
  """
  this is a description
  """
  DEFAULT
}
```
