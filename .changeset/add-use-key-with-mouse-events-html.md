---
"@biomejs/biome": minor
---

Added the HTML version of the [`useKeyWithMouseEvents`](https://biomejs.dev/linter/rules/use-key-with-mouse-events/) rule. The rule now enforces that `onmouseover` is accompanied by `onfocus` and `onmouseout` is accompanied by `onblur` in HTML, Vue, Svelte, and Astro files.

```html
<!-- Invalid: onmouseover without onfocus -->
<div onmouseover="handleMouseOver()"></div>

<!-- Valid: onmouseover paired with onfocus -->
<div onmouseover="handleMouseOver()" onfocus="handleFocus()"></div>
```
