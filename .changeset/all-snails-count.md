---
"@biomejs/biome": patch
---

Fixed [#13317](https://github.com/biomejs/biome/issues/13317): [`noDuplicateSelectors`](https://biomejs.dev/linter/rules/no-duplicate-selectors/) and [`noDescendingSpecificity`](https://biomejs.dev/linter/rules/no-descending-specificity/) no longer report false positives for rules inside separate Tailwind `@utility`, `@variant`, and `@custom-variant` blocks. Each block is now treated as an independent at-rule context while keeping nested `&` resolution intact.

The following snippet is no longer flagged as a duplicate:

```css
@utility fade {
  color: red;
  &:where([dir="rtl"], [dir="rtl"] *) {
    color: blue;
  }
}

@utility shimmer {
  color: green;
  &:where([dir="rtl"], [dir="rtl"] *) {
    color: yellow;
  }
}
```
