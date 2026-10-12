---
"@biomejs/biome": patch
---

Added the nursery rule [`useBlockStatementOverIife`](https://biomejs.dev/linter/rules/use-block-statement-over-iife/), which reports functions that are called immediately when a block statement can replace them. Biome can replace these functions with a block statement.

**Invalid:**

```js
(() => {
    const value = getValue();
    run(value);
})();
```
