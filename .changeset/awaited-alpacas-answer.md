---
"@biomejs/biome": patch
---

Added the nursery rule [`useAwaitThenable`](https://biomejs.dev/linter/rules/use-await-thenable/), which enforces that `await` is only used on Promise values.

#### Invalid

```js
await 'value';

const createValue = () => 'value';
await createValue();
```

#### Caution

This is a first iteration of the rule, and does not yet detect generic ["thenable"](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise#thenables) values.
