---
"@biomejs/biome": patch
---

Fixed [`noDescendingSpecificity`](https://biomejs.dev/linter/rules/no-descending-specificity/) computing the wrong specificity for nested selectors, and ignoring nested selectors inside `@container` and `@starting-style` blocks. The following code is now reported, because `html body & .d` resolves to `html body .x .y .d`, which is more specific than `.x .y .d`:

```css
.x {
  .y {
    html body & .d {}
  }
}
.x .y .d {}
```
