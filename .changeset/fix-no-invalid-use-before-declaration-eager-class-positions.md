---
"@biomejs/biome": patch
---

Fixed [#11945](https://github.com/biomejs/biome/issues/11945): [`noInvalidUseBeforeDeclaration`](https://biomejs.dev/linter/rules/no-invalid-use-before-declaration/) now reports uses of a class or variable in the parts of a class that run while the class is being defined, such as the `extends` clause, computed member names, static field initializers, static blocks and decorators.

```js
class A extends B {} // now reported
class C {
	static p = B; // now reported
	p = B; // still fine, instance fields run on `new`
}
class B {}
```
