---
"@biomejs/biome": patch
---

Biome's type inference now narrows types inside `switch` cases over a discriminant, and inside string equality guards such as `if (x === "value")` ([#8333](https://github.com/biomejs/biome/issues/8333)).

For example, [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/) no longer reports `value.payload;` below. Narrowing does not apply when a preceding case can fall through:

```ts
type Value =
	| { kind: "promise"; payload: Promise<void> }
	| { kind: "plain"; payload: number };

function narrowedToPlain(value: Value) {
	switch (value.kind) {
		case "plain":
			value.payload; // no longer reported by noFloatingPromises
			break;
	}
}
```
