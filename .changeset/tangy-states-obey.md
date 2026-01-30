---
"@biomejs/biome": patch
---

Added the nursery rule [`useInputName`](https://biomejs.dev/linter/rules/use-input-name/). Require mutation arguments to be called “input”, and the input type to be called Mutation name + “Input”.

**Invalid:**

```graphql
type Mutation {
  SetMessage(message: String): String
}
```
