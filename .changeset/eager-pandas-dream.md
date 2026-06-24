---
"@biomejs/biome": patch
---

[`noMisleadingReturnType`](https://biomejs.dev/linter/rules/no-misleading-return-type/) now reports misleading return annotations written as generic type alias unions.

```ts
type Maybe<T> = T | null;
// `getName` only ever returns a string, so `Maybe<string>` is misleading.
function getName(): Maybe<string> {
	return "biome";
}
```
