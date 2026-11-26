---
"@biomejs/biome": patch
---

Improved type inference for `Promise`. Biome now knows every static member that the TypeScript standard library declares for it, and the types of the `resolve` and `reject` callbacks passed to a `new Promise()` executor. For example, [`noFloatingPromises`](https://biomejs.dev/linter/rules/no-floating-promises/) now reports the unhandled Promise created by `Promise.withResolvers()`:

```ts
Promise.withResolvers().promise;
```
