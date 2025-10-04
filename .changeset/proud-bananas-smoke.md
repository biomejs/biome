---
"@biomejs/biome": patch
---

Added the new lint rule, [`useSpread`](https://biomejs.dev/linter/rules/use-consistent-arrow-return/), ported from the ESLint rule [`prefer-spread`](https://eslint.org/docs/latest/rules/prefer-spread).

This rule enforces the use of the **spread syntax** (`...`) over `Function.prototype.apply()` when calling variadic functions, as spread syntax is generally more concise and idiomatic in modern JavaScript (ES2015+).

The rule provides a safe fix.

#### Invalid

```js
Math.max.apply(Math, args);
foo.apply(undefined, args);
obj.method.apply(obj, args);
```

#### Valid

```js
Math.max(...args);
foo(...args);
obj.method(...args);

// Allowed: cases where the `this` binding is intentionally changed
foo.apply(otherObj, args);
```
