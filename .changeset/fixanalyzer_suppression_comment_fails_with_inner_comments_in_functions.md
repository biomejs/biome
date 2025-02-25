---
"@biomejs/biome": patch
---

Suppression comment should not fail with inner comments in functions.

The following code:

```ts
// biome-ignore lint/complexity/useArrowFunction: not work
const foo0 = function (bar: string) {
	// biome-ignore lint/style/noParameterAssign: work
	bar = "baz";
};
```

The suppression comment `// biome-ignore lint/style/noParameterAssign: work` will not be invalid.
