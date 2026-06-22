---
"@biomejs/biome": patch
---

Fixed [`useAwaitThenable`](https://biomejs.dev/linter/rules/use-await-thenable/) false positive when awaiting a custom thenable that is not the global `Promise`. A value with a callable `then` member is now recognized as awaitable.

```ts
interface Thenable<T> { then(onfulfilled: (value: T) => void): void; }
declare const t: Thenable<number>;
async function f() {
  await t;
}
```
