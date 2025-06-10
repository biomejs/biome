---
"@biomejs/biome": minor
---

Added the rule [`noMisusedPromises`](https://biomejs.dev/linter/rules/no-misused-promises/).

It signals `Promise`s in places where conditionals or iterables are expected.

## Invalid examples

```ts
const promise = Promise.resolve('value');

// Using a `Promise` as conditional is always truthy:
if (promise) { /* ... */ }

// Spreading a `Promise` has no effect:
console.log({ foo: 42, ...promise });
```

## Valid examples

```ts
const promise = Promise.resolve('value');

if (await promise) { /* ... */ }

console.log({ foo: 42, ...(await promise) });
```
