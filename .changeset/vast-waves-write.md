---
"@biomejs/biome": patch
---

Fixed [#6617](https://github.com/biomejs/biome/issues/6617): Improved [`useIterableCallbackReturn`](https://biomejs.dev/linter/rules/use-iterable-callback-return/) to correctly handle arrow functions with a single-expression `void` body.

```js
[].forEach(() => void null);
```
