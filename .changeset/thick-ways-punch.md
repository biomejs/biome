---
"@biomejs/biome": patch
---

Biome's type inference now narrows the type of a variable inside the consequent of user-defined type guards such as `if (isFoo(x))` ([#8333](https://github.com/biomejs/biome/issues/8333)).

For example, [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/) now reports the call below:

```ts
type Task = { run: () => Promise<void> };

function isTask(value: unknown): value is Task {
	return typeof value === "object" && value !== null && "run" in value;
}

function guardedCall(value: unknown) {
	if (isTask(value)) {
		value.run(); // now reported by noFloatingPromises
	}
}
```
