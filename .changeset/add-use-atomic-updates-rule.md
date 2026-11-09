---
"@biomejs/biome": patch
---

Added the nursery rule [`useAtomicUpdates`](https://biomejs.dev/linter/rules/use-atomic-updates/), which reports assignments in async functions and generators that can be based on an outdated value because the function paused at an `await` or `yield` after reading it.

```js
let total = 0;
async function addToTotal(getAmount) {
    total += await getAmount(); // reported
}
```
