---
"@biomejs/biome": minor
---

Added the new assist action [`useGraphqlSortedVariables`](https://biomejs.dev/assist/actions/use-graphql-sorted-variables/), which sorts the variable definitions of GraphQL operations.

**Invalid**:

```graphql
query GetUser($userId: ID!, $includePosts: Boolean = false, $after: String) {
  user(id: $userId) {
    name
  }
}
```
