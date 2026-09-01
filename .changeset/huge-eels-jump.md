---
"@biomejs/biome": patch
---

Added the nursery rule [`useModernMathApis`](https://biomejs.dev/linter/rules/use-modern-math-apis/). The rule reports legacy mathematical patterns that have direct modern `Math` equivalents.

```js
Math.sqrt(a * a + b * b);
```
