---
"@biomejs/biome": patch
---

Fixed [#9997](https://github.com/biomejs/biome/issues/9997): [`noDuplicateSelectors`](https://biomejs.dev/linter/rules/no-duplicate-selectors/) no longer reports false positives for selectors inside `@scope` queries. Biome now treats `@scope` as a separate at-rule context, like `@media`, `@supports`, `@container`, and `@starting-style`.

The following snippet is no longer flagged as a duplicate:

```css
.Example {
  padding: 0;
}

@scope (.theme-dark) {
  .Example {
    color: white;
  }
}
```
