---
"@biomejs/biome": patch
---

Fixed [#9867](https://github.com/biomejs/biome/issues/9867): the [`noDuplicateFields`](https://biomejs.dev/linter/rules/no-duplicate-fields/) rule now detects fields that are already provided by a fragment spread in the same selection set.

For example, the following code now triggers a diagnostic because `name` is selected both directly and through `...MemberFields`:

```graphql
fragment MemberFields on Member {
  id
  name
}

query {
  member {
    ...MemberFields
    name
  }
}
```
