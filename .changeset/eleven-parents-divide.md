---
"@biomejs/biome": patch
---

Added the nursery rule [`useLoneAnonymousOperation`](https://biomejs.dev/linter/rules/use-lone-anonymous-operation/). Disallow anonymous operations when more than one operation specified in document.

**Invalid:**

```graphql
query {
  fieldA
}

query B {
  fieldB
}
```
