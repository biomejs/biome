---
"@biomejs/biome": patch
---

Fixed [#8338](https://github.com/biomejs/biome/issues/8292): Ignore unknownTypeSelector check when root under view transition pseudo elements.

#### Example

```css
::view-transition-old(root),
::view-transition-new(root) {
	z-index: 1;
}
```

