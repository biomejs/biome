---
"@biomejs/biome": patch
---

Fixed [#9568](https://github.com/biomejs/biome/issues/9568): [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/) no longer reports a false positive when calling an overloaded function and the selected overload does not return a promise.

```ts
function bestEffort(cb: () => Promise<number>): Promise<number>;
function bestEffort(cb: () => number): number;
function bestEffort(cb: () => number | Promise<number>): Promise<number> | number {
	return cb() as Promise<number> | number;
}

// This resolves to the second overload, which returns `number`, so it is no
// longer flagged as a floating promise.
bestEffort(() => 42);
```
