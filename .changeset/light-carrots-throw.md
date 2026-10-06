---
"@biomejs/biome": patch
---

Improved the accuracy of type-aware lint rules by resolving more inferred types. For example, [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/) now detects floating Promises returned by aliased callbacks and arrays of Promises created by async mapping callbacks.

The following statements are now reported:

```ts
type AsyncCallback = () => Promise<void>;
declare const callback: AsyncCallback;
callback();

[1, 2, 3].map(async value => value);
```
