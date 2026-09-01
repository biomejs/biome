---
"@biomejs/biome": patch
---

Added the recommended nursery rule [`useObserverApi`](https://biomejs.dev/linter/rules/use-observer-api/). The rule reports `resize` and `scroll` listeners that synchronously read layout and recommends `ResizeObserver` or `IntersectionObserver` instead.

```js
window.addEventListener("resize", () => element.offsetWidth);
```
