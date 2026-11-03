---
"@biomejs/biome": patch
---

Added the nursery rule [`noUselessAwait`](https://biomejs.dev/linter/rules/no-useless-await/), which reports `await` on values that are never promises, such as literals, arrays, and the results of comparisons.

```js
await [fetchUser(), fetchPosts()];
```
