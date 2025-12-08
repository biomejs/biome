---
"@biomejs/biome": patch
---

Fixed [#8338](https://github.com/biomejs/biome/issues/8338): Ignored the `noUnknownTypeSelector` check when the `root` selector is used under View Transition pseudo-elements.

**Example**

```css
::view-transition-old(root),
::view-transition-new(root) {
	z-index: 1;
}
```

