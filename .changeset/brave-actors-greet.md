---
"@biomejs/biome": patch
---

Biome's type inference now narrows the type of a variable inside the consequent of an `instanceof` guard ([#8333](https://github.com/biomejs/biome/issues/8333)).

For example, [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/) now reports the call below:

```ts
class Base {}
class Derived extends Base {
	async run(): Promise<void> {}
}

function guardedCall(instance: Base) {
	if (instance instanceof Derived) {
		instance.run(); // now reported by noFloatingPromises
	}
}
```
