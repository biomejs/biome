---
"@biomejs/biome": patch
---

Fixed [#5307](https://github.com/biomejs/biome/issues/5307), where CSS value lists were wrapped in a way that did not preserve semantic structure.

Biome now ensures that CSS value lists follow a more readable format, aligning with Prettier's behavior.

Before:

```css
* {
	box-shadow: 0 0 0 1px #fff, 0 0 0 3.2px rgba(89, 89, 235, 0.25), 0 0 0 3.2px
		rgba(89, 89, 235, 0.25), 0 0 0 3.2px red, 0 0 0 3.2px
		rgba(89, 89, 235, 0.25);
}
```

After:

```css
* {
  box-shadow:
    0 0 0 1px #fff,
    0 0 0 3.2px rgba(89, 89, 235, 0.25),
    0 0 0 3.2px rgba(89, 89, 235, 0.25),
    0 0 0 3.2px red,
    0 0 0 3.2px rgba(89, 89, 235, 0.25);
}
```
