---
"@biomejs/biome": patch
---

Biome's type inference now narrows discriminated unions inside the consequent of guards such as `if (x.kind === "tag")` ([#8333](https://github.com/biomejs/biome/issues/8333)).

For example, [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/) no longer reports `value.payload;` below, because the guard excludes the promise-carrying variant:

```ts
type Value =
	| { kind: "promise"; payload: Promise<void> }
	| { kind: "plain"; payload: number };

function narrowedToPlain(value: Value) {
	if (value.kind === "plain") {
		value.payload; // no longer reported by noFloatingPromises
	}
}
```
