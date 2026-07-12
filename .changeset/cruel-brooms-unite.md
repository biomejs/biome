---
"@biomejs/biome": patch
---

Improved Biome's array and `await` type inference for [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/). Biome now detects arrays of Promises produced from explicitly typed arrays and chained array methods, and preserves non-Promise branches when awaiting unions. For example, `noFloatingPromises` now detects this array of unhandled Promises:

```ts
declare const values: Array<number>;
values.map(async (value) => value);
```
