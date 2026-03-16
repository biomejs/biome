---
"@biomejs/biome": minor
---

Added the `useKeyWithClickEvents` a11y lint rule for HTML files (`.html`, `.vue`, `.svelte`, `.astro`). This is a port of the existing JSX rule. The rule enforces that elements with an `onclick` handler also have at least one keyboard event handler (`onkeydown`, `onkeyup`, or `onkeypress`) to ensure keyboard accessibility.

Inherently keyboard-accessible elements (`<a>`, `<button>`, `<input>`, `<select>`, `<textarea>`, `<option>`) are excluded, as are elements hidden from assistive technologies (`aria-hidden`) or with `role="presentation"` / `role="none"`.

```html
<!-- Invalid: no keyboard handler -->
<div onclick="handleClick()">Click me</div>

<!-- Valid: has keyboard handler -->
<div onclick="handleClick()" onkeydown="handleKeyDown()">Click me</div>

<!-- Valid: inherently keyboard-accessible -->
<button onclick="handleClick()">Submit</button>
```
