---
"@biomejs/biome": minor
---

Added the new assist action [`useGraphqlSortedDefinitions`](https://biomejs.dev/assist/actions/use-graphql-sorted-definitions/), which sorts the named definitions of a GraphQL document, such as types, type extensions, directives, fragments, and operations.

**Invalid**:

```graphql
type User {
  id: ID!
}

enum Role {
  ADMIN
  USER
}

directive @auth on FIELD_DEFINITION
```
