---
"@biomejs/biome": patch
---

Added the nursery rule [`useTypedIterableCallbackReturn`](https://biomejs.dev/linter/rules/use-typed-iterable-callback-return/), a type-aware alternative to [`useIterableCallbackReturn`](https://biomejs.dev/linter/rules/use-iterable-callback-return/) addressing [#8247](https://github.com/biomejs/biome/issues/8247). It accepts `forEach` callbacks that return `void`, skips custom objects with similarly named methods, and reports missing callback results.

```ts
function log(value: number): void {
  console.log(value);
}

[1, 2, 3].map(value => log(value));
```
