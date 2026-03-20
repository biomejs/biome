---
"@biomejs/biome": patch
---

Fixed [#6206](https://github.com/biomejs/biome/issues/6206): [`useUniqueElementIds`](https://biomejs.dev/linter/rules/use-unique-element-ids/) no longer reports false positives for SVG elements. SVG `id` attributes are scoped to the SVG document fragment and do not require global uniqueness, unlike HTML element ids.

```jsx
// Now accepted — SVG id is scoped to the SVG document
<svg>
  <pattern id="my-pattern" />
  <rect fill="url(#my-pattern)" />
</svg>
```
