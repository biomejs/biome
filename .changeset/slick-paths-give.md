---
"@biomejs/biome": patch
---

Added the nursery rule [`noJsxLeakedComment`](https://biomejs.dev/linter/rules/no-jsx-leaked-comment), which prevents comments from being inserted as JSX text nodes.

**Invalid**:

```jsx
<div>
  // This will be rendered as text
  /* This will also be rendered as text */
</div>
```
