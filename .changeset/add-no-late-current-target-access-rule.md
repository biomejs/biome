---
"@biomejs/biome": patch
---

Added the nursery rule [`noLateCurrentTargetAccess`](https://biomejs.dev/linter/rules/no-late-current-target-access/), which reports reads of `event.currentTarget` that run after the browser has finished handling the event, when `currentTarget` is `null`.

```js
element.addEventListener("click", async (event) => {
  await save();
  event.currentTarget.disabled = false;
});
```
