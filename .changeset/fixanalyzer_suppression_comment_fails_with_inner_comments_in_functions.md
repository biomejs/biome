---
"@biomejs/biome": patch
---

Fixed [#4714](https://github.com/biomejs/biome/pull/4714): Suppression comments no longer fail on functions that themselves contain suppression comments.

This now works correctly:

```ts
// biome-ignore lint/complexity/useArrowFunction: this suppression now works
const foo0 = function (bar: string) {
	// biome-ignore lint/style/noParameterAssign: even if there are other suppressions inside
	bar = "baz";
};
```
