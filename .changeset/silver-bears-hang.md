---
"@biomejs/biome": patch
---

Fixed [#10045](https://github.com/biomejs/biome/issues/10045): the CSS formatter no longer compounds indentation inside nested functional pseudo-classes such as `:not(:where(...))`, `:is(:where(...))`, and similar combinations. The same fix also removes one level of unnecessary indentation that was added inside any pseudo-class function whose argument list wrapped onto multiple lines, including `:nth-child(... of ...)`, `::part(...)`, and `:active-view-transition-type(...)`.
The following snippet is now correctly formatted, matching Prettier.

```css
input:not(
	:where(
		[type="submit"],
		[type="checkbox"],
		[type="radio"],
		[type="button"],
		[type="reset"]
	)
) {
	inline-size: 100%;
}
```

