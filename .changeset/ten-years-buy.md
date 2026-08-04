---
"@biomejs/biome": patch
---

Fixed [#11214](https://github.com/biomejs/biome/issues/11214): [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/) no longer reports type parameters declared on function overload signatures as unused.

```ts
function someFn<MyGeneric>();
function someFn<MyGeneric>() {
	const a: MyGeneric = returnAny();
	console.log(a);
}
```
