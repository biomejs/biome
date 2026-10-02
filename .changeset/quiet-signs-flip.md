---
"@biomejs/biome": patch
---

Added the nursery rule [`useUnaryMinus`](https://biomejs.dev/linter/rules/use-unary-minus/), which reports multiplying or dividing by `-1` instead of using the unary minus operator.

```js
const inverted = value * -1;
```
