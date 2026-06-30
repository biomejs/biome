---
"@biomejs/biome": patch
---

Fixed [#8269](https://github.com/biomejs/biome/issues/8269): the CSS parser now accepts Tailwind `@variant` and `@utility` names that start with a digit, such as the `2xl` breakpoint.

```css
@utility container {
	@variant 2xl {
		max-width: 1400px;
	}
}
```
