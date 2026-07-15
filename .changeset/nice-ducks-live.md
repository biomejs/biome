---
"@biomejs/biome": patch
---

Improved Biome's member type inference for [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/). Biome now resolves intersection members, computed properties, inherited static methods, and generic class or object methods that access properties through `this`. For example, `noFloatingPromises` now detects the inherited Promise-returning static method:

```ts
class Base {
	static read(): Promise<void> {
		return Promise.resolve();
	}
}
class Derived extends Base {}
Derived.read();
```
