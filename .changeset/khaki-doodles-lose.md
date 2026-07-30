---
"@biomejs/biome": patch
---

Fixed [#10514](https://github.com/biomejs/biome/issues/10514): [`noLeakedRender`](https://biomejs.dev/linter/rules/no-leaked-render/) no longer reports ternary expressions whose alternate is a variable. Only `undefined` alternates are reported now, so the following code is valid:

```jsx
<div>{isMobile ? null : decorations}</div>
```

The logical AND cases reported in the same issue are unchanged, since determining whether the left-hand side can be `0` or `""` requires type information the rule does not have.
