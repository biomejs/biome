---
"@biomejs/biome": patch
---

Added the nursery rule [`noLateEventControl`](https://biomejs.dev/linter/rules/no-late-event-control/), which reports calls to `preventDefault()`, `stopPropagation()`, and `stopImmediatePropagation()` that run after the browser has finished handling the event, where they have no effect.

```js
element.addEventListener("click", async (event) => {
  await save();
  event.preventDefault();
});
```
