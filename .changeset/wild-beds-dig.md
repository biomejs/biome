---
"@biomejs/biome": patch
---

Fixed false positives in [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/), [`noMisusedPromises`](https://biomejs.dev/linter/rules/no-misused-promises/), and [`useAwaitThenable`](https://biomejs.dev/linter/rules/use-await-thenable/) when Promise or thenable inference cannot complete. These rules now suppress diagnostics instead of treating incomplete type information as a definite result. For example, `useAwaitThenable` no longer reports `await value` when the value's thenability is unknown:

```ts
declare const value: unknown;

async function consume() {
	await value;
}
```
