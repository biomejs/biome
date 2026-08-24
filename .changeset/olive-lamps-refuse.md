---
"@biomejs/biome": patch
---

Biome's type inference now narrows values of class instance and interface types out of falsy branches, and classifies interface-typed values as always truthy, as it already did for object and class types, following TypeScript's narrowing semantics ([#8333](https://github.com/biomejs/biome/issues/8333)). Values of generic type parameters are still conservatively kept, since they may be instantiated with falsy types.

For example, [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) now reports the inner condition below:

```ts
function example(p: Promise<void> | undefined) {
	if (!p) {
		// `p` is narrowed to `undefined` here.
		if (p) { // now reported as always falsy
			p;
		}
	}
}
```

As a consequence, a logical expression whose left side is a value of an interface type now short-circuits to that value's type, like it already did for object and class types.
