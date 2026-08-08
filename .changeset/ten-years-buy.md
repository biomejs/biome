---
"@biomejs/biome": patch
---

Fixed [#11214](https://github.com/biomejs/biome/issues/11214): [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/) no longer reports type parameters declared on function or class method overload signatures that have an implementation.

```ts
function someFn<MyGeneric>();
function someFn<MyGeneric>() {
	const a: MyGeneric = returnAny();
	console.log(a);
}

class C {
	method<MyGeneric>(): void;
	method<MyGeneric>(a?: MyGeneric): void {
		console.log(a);
	}
}
```

Type parameters on signatures that never get an implementation, such as abstract methods and interface members, are still reported.
