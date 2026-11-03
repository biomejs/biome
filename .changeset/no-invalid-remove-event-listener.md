---
"@biomejs/biome": patch
---

Added the new nursery rule [`noInvalidRemoveEventListener`](https://biomejs.dev/linter/rules/no-invalid-remove-event-listener/), which reports calls to `removeEventListener()` that pass an inline function or the result of `.bind()`. These create a new function, so the listener that was added is never removed.

```js
window.removeEventListener("click", listener.bind(window));
```
