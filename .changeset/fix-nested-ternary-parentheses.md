---
"@biomejs/biome": patch
---

Fixed [#8045](https://github.com/biomejs/biome/issues/8045): The [`noNestedTernary`](https://biomejs.dev/linter/rules/no-nested-ternary/) rule now correctly detects nested ternary expressions even when they are wrapped in parentheses (e.g. `foo ? (bar ? 1 : 2) : 3`).

Previously, the rule would not flag nested ternaries like `foo ? (bar ? 1 : 2) : 3` because the parentheses prevented detection. The rule now looks through parentheses to identify nested conditionals.

**Previously not detected (now flagged):**

```js
const result = foo ? (bar ? 1 : 2) : 3;
```

**Still valid (non-nested with parentheses):**

```js
const result = (foo ? bar : baz);
```
