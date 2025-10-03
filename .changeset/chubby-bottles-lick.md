---
"@biomejs/biome": patch
---

Added the rule [`useDescriptionStyle`](https://biomejs.dev/linter/rules/use-description-style/), requiring all descriptions to follow the same style (either block or inline).

##### Invalid

```graphql
enum EnumValue {
  "this is a description"
  DEFAULT
}
```

##### Valid

```graphql
enum EnumValue {
  """
  this is a description
  """
  DEFAULT
}
```
