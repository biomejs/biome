---
"@biomejs/biome": patch
---

Partially fix [#7583](https://github.com/biomejs/biome/issues/7583).
[`organizeImports`](https://biomejs.dev/assist/actions/organize-imports/) now
sorts named specifiers inside bare exports.
This fix doesn't merge adjacent bare exports.

```diff
- export { b, a };
+ export { a, b };
```
