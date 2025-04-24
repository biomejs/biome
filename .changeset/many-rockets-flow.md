---
"@biomejs/biome": minor
---

Added the new rule [`useIterableCallbackReturn`](https://biomejs.dev/linter/rules/use-iterable-callback-return), which enforces consistent return values in iterable callbacks.

The following methods require a return in their callback:

- `every`
- `filter`
- `find`
- `findIndex`
- `findLast`
- `findLastIndex`
- `flatMap`
- `map`
- `reduce`
- `reduceRight`
- `some`
- `sort`
- `toSorted`
— `from` (when called on `Array`)

Method `forEach` do not require the callback to return a value.

Examples:

```js
[].map(() => {
    // Missing return value
});
```

```js
[].forEach(() => {
    return 1; // Not required
});
```
