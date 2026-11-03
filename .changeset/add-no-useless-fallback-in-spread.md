---
"@biomejs/biome": patch
---

Added the nursery rule [`noUselessFallbackInSpread`](https://biomejs.dev/linter/rules/no-useless-fallback-in-spread/), which reports empty object fallbacks when spreading a value into an object. Spreading `null`, `undefined`, or another falsy value into an object adds no properties, so the fallback has no effect.

```js
const object = { ...(options || {}) };
```
