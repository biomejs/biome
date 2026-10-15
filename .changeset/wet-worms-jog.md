---
"@biomejs/biome": patch
---

Fixed parsing and formatting of `:heading()` selectors with integer arguments in CSS and SCSS. The [`noUnknownPseudoClass`](https://biomejs.dev/linter/rules/no-unknown-pseudo-class/) rule recognizes both `:heading` and `:heading()`, and the function contributes one class to selector specificity regardless of its argument count.

```css
.article :heading(1, 2, 3) {
  text-wrap: balance;
}
```
