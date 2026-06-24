---
"@biomejs/biome": patch
---

[`useConsistentEnumValueType`](https://biomejs.dev/linter/rules/use-consistent-enum-value-type/) no longer reports a false positive when a numeric enum member value is typed through a generic type alias.

```ts
type Id<T> = T;
const one: Id<1> = 1;
enum NumericOnly {
	A = one,
	B = 2,
}
```
