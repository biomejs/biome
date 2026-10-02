---
"@biomejs/biome": patch
---

Fixed [#11945](https://github.com/biomejs/biome/issues/11945): [`noInvalidUseBeforeDeclaration`](https://biomejs.dev/linter/rules/no-invalid-use-before-declaration/) now reports references in the positions that a class declaration evaluates before it initializes the class binding.

A class definition evaluates its heritage clause, its computed member names, its static field initializers and its static blocks while it is created. These references now throw a `ReferenceError` and are reported:

```js
class A extends B {}
class C {
	[B.name]() {}
}
class D {
	static p = B;
}
class E {
	static {
		B;
	}
}
class B {}
```

Instance field initializers and the bodies of methods, accessors and constructors run later, so a reference to a class declared after them stays valid.
