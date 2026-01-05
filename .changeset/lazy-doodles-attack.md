---
"@biomejs/biome": patch
---

Added the nursery rule [`useLoneExecutableDefinition`](https://biomejs.dev/linter/rules/use-lone-executable-definition/). Require queries, mutations, subscriptions or fragments to be located in separate files.

**Invalid:**

```graphql
query Foo {
  id
}

fragment Bar on Baz {
  id
}
```
