---
"@biomejs/biome": patch
---

Fixed [#13049](https://github.com/biomejs/biome/issues/13049): [`useMathMinMax`](https://biomejs.dev/linter/rules/use-math-min-max/) no longer reports `reduce()` callbacks whose initial value is non-numeric when the array isn't a literal, such as the result of `.map()`.

```js
const strings = ["foo", "bar"].map((a) => a);
const largest = strings.reduce((a, b) => (a > b ? a : b), "");
```
