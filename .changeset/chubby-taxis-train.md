---
"@biomejs/biome": patch
---

Added a new nursery rule [`noRedundantTypeArguments`](https://biomejs.dev/linter/rules/no-redundant-type-arguments/), that disallows explicit type arguments when they match a default type parameter or can be inferred from the surrounding call.

For example, the following snippet now triggers the rule:

```ts
function f<T = number>() {}
f<number>();
```
