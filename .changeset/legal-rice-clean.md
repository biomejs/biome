---
"@biomejs/biome": patch
---

Added the [`noEqualsToNull`](https://biomejs.dev/linter/rules/no-equals-to-null) rule, which enforces the use of `===` and `!==` for comparison with `null` instead of `==` or `!=`.

**Invalid:**

```js
foo == null;
foo != null;
```

**Valid:**

```js
foo === null;
foo !== null;
```
