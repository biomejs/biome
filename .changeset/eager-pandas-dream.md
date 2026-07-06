---
"@biomejs/biome": patch
---

[`noMisleadingReturnType`](https://biomejs.dev/linter/rules/no-misleading-return-type/) now reports misleading return annotations written as generic type alias unions.

```ts
type Maybe<T> = T | null;
function genericAliasUnion(): Maybe<string> { return "hello"; }
```
