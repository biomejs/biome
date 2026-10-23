---
"@biomejs/biome": patch
---

Fixed [`noDescendingSpecificity`](https://biomejs.dev/linter/rules/no-descending-specificity/) to compare resolved nested selector targets. Biome no longer reports nested selectors that resolve to different targets, detects equivalent explicit and nested targets, and emits one diagnostic when a parent selector list expands the same nested selector multiple times.

For example, the following selectors are no longer reported because their final targets are `.column` and `code`, respectively:

```css
.card {
  .column {
    :root[data-theme="dark"] & {}
  }

  code {
    :root[data-theme="dark"] & {}
  }
}
```


