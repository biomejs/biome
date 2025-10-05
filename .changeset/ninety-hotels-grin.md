---
"@biomejs/biome": patch
---

Added the rule [`useDeprecatedDate`](https://biomejs.dev/linter/rules/use-deprecated-date/), which makes a deprecation date required for the graphql `@deprecated` directive.

##### Invalid

```graphql
query {
  member @deprecated(reason: "Use `members` instead") {
    id
  }
}
```

##### Valid

```graphql
query {
  member @deprecated(reason: "Use `members` instead", deletionDate: "2099-12-25") {
    id
  }
}
```
