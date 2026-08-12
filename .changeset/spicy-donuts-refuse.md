---
"@biomejs/biome": patch
---

Biome's type inference now narrows the type of a variable after an assignment, using the type of the nearest preceding assignment in the same block ([#8333](https://github.com/biomejs/biome/issues/8333)).

For example, [`noUnnecessaryConditions`](https://biomejs.dev/linter/rules/no-unnecessary-conditions/) now reports the condition below:

```ts
function example(x: string | undefined) {
	x = "on";
	if (x) { // now reported by noUnnecessaryConditions
		x;
	}
}
```
