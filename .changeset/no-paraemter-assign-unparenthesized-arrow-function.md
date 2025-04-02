---
"@biomejs/biome": patch
---

[noParameterAssign](https://biomejs.dev/linter/rules/no-parameter-assign) now reports reassigned parameter of unparenthesized arrow functions.

The following code is now reported as invalid.

```js
const f = param => {
  param = {}; // Reassigning a function parameter is confusing.
};
```

Fixed [#5409](https://github.com/biomejs/biome/issues/5409).
