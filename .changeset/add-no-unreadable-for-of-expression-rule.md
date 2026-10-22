---
"@biomejs/biome": patch
---

Added the new nursery rule [`noUnreadableForOfExpression`](https://biomejs.dev/linter/rules/no-unreadable-for-of-expression/), which reports `for...of` and `for await...of` loops that go through the result of a complex expression instead of a variable, a property, or a short call.

```js
for (const item of getItems(createArgument(seed))) {} // reported
```
