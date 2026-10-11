---
"@biomejs/biome": patch
---

Added the nursery rule [`noPromiseExecutorReturn`](https://biomejs.dev/linter/rules/no-promise-executor-return/), which disallows returning values from the function passed to `new Promise()`, because `new Promise()` ignores them.

```js
new Promise((resolve) => resolve(1));
```
