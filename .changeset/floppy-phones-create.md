---
"@biomejs/biome": patch
---

Added the nursery rule [`useUniqueFieldDefinitionNames`](https://biomejs.dev/linter/rules/use-unique-field-definition-names/). Require all fields of a type to be unique.

**Invalid:**

```graphql
type SomeObject {
  foo: String
  foo: String
}
```
