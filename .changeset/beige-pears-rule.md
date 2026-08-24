---
"@biomejs/biome": patch
---

Biome's type inference now narrows the type of a variable inside the consequent of truthiness guards such as `if (x)` and `if (!x)` ([#8333](https://github.com/biomejs/biome/issues/8333)).

For example, [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) now reports the inner condition below as always truthy:

```ts
function f(x: { a: number } | null) {
	if (x) {
		if (x) { // now reported by noUnnecessaryConditions
			x.a;
		}
	}
}
```
