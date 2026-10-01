---
"@biomejs/biome": minor
---

Added the new assist action [`useSortedVariables`](https://biomejs.dev/assist/actions/use-sorted-variables/), which sorts the variable definitions of GraphQL operations.

**Invalid**:

```graphql
query GetUser($userId: ID!, $includePosts: Boolean = false, $after: String) {
  user(id: $userId) {
    name
  }
}
```
