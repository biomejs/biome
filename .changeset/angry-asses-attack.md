---
"@biomejs/biome": minor
---

The rule [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-misused-promises/)
can now detect floating arrays of `Promise`s.

## Invalid examples

```ts
// This gets flagged because the Promises are not handled.
[1, 2, 3].map(async (x) => x + 1);
```

## Valid examples

```ts
await Promise.all([1, 2, 3].map(async (x) => x + 1));
```
