---
"@biomejs/biome": minor
---

Added the new assist action [`useGraphqlSortedArguments`](https://biomejs.dev/assist/actions/use-graphql-sorted-arguments/), which sorts the arguments of GraphQL fields and directives, and the arguments declared by field and directive definitions.

**Invalid**:

```graphql
query {
  users(orderBy: NAME, first: 10, after: "cursor") {
    name
  }
}
```
