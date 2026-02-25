---
"@biomejs/biome": patch
---

Extended the nursery rule [`useNullishCoalescing`](https://biomejs.dev/linter/rules/use-nullish-coalescing/) to detect ternary expressions that perform explicit nullish checks and suggest rewriting them with `??`.

The rule now flags ternary patterns like:

```js
const v = a !== null ? a : 'default';
const v = a === undefined ? 'default' : a;
const v = a != null ? a : 'default';
const v = a === null || a === undefined ? 'default' : a;
```

All of these can be simplified to `a ?? 'default'`.

Added a new `ignoreTernaryTests` option (default: `false`) to allow disabling this check independently of the existing logical OR detection.
