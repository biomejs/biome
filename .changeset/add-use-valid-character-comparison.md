---
"@biomejs/biome": patch
---

Added the new nursery rule [`useValidCharacterComparison`](https://biomejs.dev/linter/rules/use-valid-character-comparison/), which reports comparisons between a single character and a string that has more than one character. Such comparisons are always `false`, or always `true` with `!==` and `!=`.

```js
text.charAt(0) === "ab";
```
