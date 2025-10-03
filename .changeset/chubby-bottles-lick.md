---
"@biomejs/biome": patch
---

Added the rule [`useDescriptionStyle`](https://biomejs.dev/linter/rules/use-description-style/), require all comments to follow the same format (either block or inline)

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
