---
"@biomejs/biome": patch
---

Fixed [`noDuplicateSelectors`](https://biomejs.dev/linter/rules/no-duplicate-selectors/) ignoring nested selectors inside `@container` and `@starting-style` blocks. The following duplicate is now reported:

```css
.card {
  @container (min-width: 1px) {
    & > p {}
    & > p {}
  }
}
```
