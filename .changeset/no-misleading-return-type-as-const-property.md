---
"@biomejs/biome": patch
---

[`noMisleadingReturnType`](https://biomejs.dev/linter/rules/no-misleading-return-type/) now detects misleading return type annotations when object literal properties are initialized with `as const`.

This function is now reported because the return annotation widens a property initialized with `as const`:

```ts
function f(): { value: string } {
	return { value: "text" as const };
}
```
