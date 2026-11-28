---
"@biomejs/biome": patch
---

Fixed [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/) and [`noMisusedPromises`](https://biomejs.dev/linter/rules/no-misused-promises/) not recognizing the `PromiseLike` that the methods of a `PromiseLike` value return. For example, `noFloatingPromises` now reports the following code:

```ts
declare const thenable: PromiseLike<number>;
thenable.then((value) => value);
```
