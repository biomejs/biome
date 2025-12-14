---
"@biomejs/biome": patch
---

Fixed [#8179](https://github.com/biomejs/biome/issues/8179): The [`useConsistentArrowReturn`](https://biomejs.dev/linter/rules/use-consistent-arrow-return/) rule now correctly handles multiline expressions in its autofix when the `style` option is set to `"always"`.

Previously, the autofix would incorrectly place a newline after the `return` keyword, causing unexpected behavior.

Example:
```js
const foo = (l) =>
  l
    .split('\n')
```
Now correctly autofixes to:
```diff
const foo = (l) => {
-   return
-   l.split('\n');
+   return l.split('\n');
}
```
