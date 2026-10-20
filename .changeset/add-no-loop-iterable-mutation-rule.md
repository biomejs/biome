---
"@biomejs/biome": patch
---

Added the nursery rule [`noLoopIterableMutation`](https://biomejs.dev/linter/rules/no-loop-iterable-mutation/), which reports changes to an array, `Set`, or `Map` while a `for...of` loop iterates over it. Such changes can make the loop skip items, visit items more than once, or never end.

```js
for (const item of items) {
  items.push(item.clone());
}
```
