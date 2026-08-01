---
"@biomejs/biome": patch
---

Fixed [#8333](https://github.com/biomejs/biome/issues/8333): Biome's type inference now narrows the type of a variable inside the consequent of an `if (typeof x === "...")` guard.

Type-aware rules see the narrowed type. For example, [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/) no longer reports `x;` below, while [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) now reports `if (y)` as always truthy:

```ts
function typed(x: number | Promise<void>, y: number | (() => void)) {
	if (typeof x === "number") {
		x; // no longer reported by noFloatingPromises
	}
	if (typeof y === "function") {
		if (y) { // now reported by noUnnecessaryConditions
			y();
		}
	}
}
```
