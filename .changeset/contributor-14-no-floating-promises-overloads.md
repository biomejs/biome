---
"@biomejs/biome": patch
---

Fixed [#9568](https://github.com/biomejs/biome/issues/9568): `noFloatingPromises` now respects adjacent TypeScript overload signatures when matching callback-based calls.

Previously, overloaded helpers could be reported as floating promises whenever any overload returned a `Promise`, even if the selected overload for the current callback returned a synchronous value:

```ts
export function bestEffort<T>(cb: () => Promise<T>): Promise<T | undefined>;
export function bestEffort<T>(cb: () => T): T | undefined;
export function bestEffort<T>(cb: (() => T) | (() => Promise<T>)) {
	return cb();
}

bestEffort(() => 1);
```

That synchronous call is no longer flagged, while the async callback overload still reports correctly.
