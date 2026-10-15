---
"@biomejs/biome": patch
---

The CSS parser now accepts an identifier that ends in `n` as the argument of `:nth-child` and `:nth-of-type`, such as `:nth-child(ln)`, instead of reporting a syntax error.

```css
:nth-child(ln) {
	color: red;
}
```
