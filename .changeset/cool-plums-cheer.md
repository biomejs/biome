---
"@biomejs/biome": patch
---

Added the nursery rule [`noRootType`](https://biomejs.dev/linter/rules/no-root-type).
Disallow the usage of specified root types. (e.g. `mutation` and/or `subscription`)

**Invalid:**

```json
{
  "options": {
    "disallow": ["mutation"]
  }
}
```

```graphql
type Mutation {
  SetMessage(message: String): String
}
```
