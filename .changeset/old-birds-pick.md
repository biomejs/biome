---
"@biomejs/biome": patch
---

Fixed [#10492](https://github.com/biomejs/biome/issues/10492): Biome no longer crashes with a stack overflow on certain code when a type-aware rule such as [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/), [`noMisusedPromises`](https://biomejs.dev/linter/rules/no-misused-promises/), or [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) is enabled. For example, the following code used to crash Biome:

```js
function f(visitor) {
	let ctrl = visitor();
	for (const x of [0]) ctrl = ctrl();
}
```
