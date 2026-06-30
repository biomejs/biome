---
"@biomejs/biome": patch
---

Fixed [#7533](https://github.com/biomejs/biome/issues/7533): [`noDescendingSpecificity`](https://biomejs.dev/linter/rules/no-descending-specificity/) no longer reports false positives across `@layer` blocks. Specificity only matters within a single cascade layer, so selectors in different layers are no longer compared.

```css
@layer one {
	b a { color: green; }
}

@layer two {
	a { color: blue; }
}
```
