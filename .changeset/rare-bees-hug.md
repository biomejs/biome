---
"@biomejs/biome": patch
---

Fixed [#7877](https://github.com/biomejs/biome/issues/7877): Range suppressions now handle suppressed categories properly.

**Valid:**

```js
// biome-ignore-start lint: explanation
const foo = 1;
// biome-ignore-end lint: explanation
```
