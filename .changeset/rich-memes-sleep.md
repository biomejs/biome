---
"@biomejs/biome": patch
---

Added the GraphQL nursery rule [`useUniqueGraphqlOperationName`](https://biomejs.dev/linter/rules/use-unique-graphql-operation-name). This rule ensures that all GraphQL operations within a document have unique names.

**Invalid:**
```graphql
query user {
  user {
    id
  }
}

query user {
  user {
    id
    email
  }
}
```

**Valid:**
```graphql
query user {
  user {
    id
  }
}

query userWithEmail {
  user {
    id
    email
  }
}
```
