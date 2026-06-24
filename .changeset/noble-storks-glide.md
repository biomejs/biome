---
"@biomejs/biome": patch
---

[`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) now detects unnecessary optional chaining on non-nullish values typed through a generic type alias.

```ts
type Id<T> = T;
function readLength(text: Id<string>) {
	return text?.length;
}
```
