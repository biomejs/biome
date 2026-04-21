---
"@biomejs/biome": minor
---

Added HTML support for the [`noAriaHiddenOnFocusable`](https://biomejs.dev/linter/rules/no-aria-hidden-on-focusable/) accessibility lint rule, which enforces that `aria-hidden="true"` is not set on focusable elements. Focusable elements include native interactive elements (`<button>`, `<input>`, `<select>`, `<textarea>`), elements with `href` (`<a>`, `<area>`), elements with `tabindex >= 0`, and editing hosts (`contenteditable`). Includes an unsafe fix to remove the `aria-hidden` attribute.

```html
<!-- Invalid: aria-hidden on a focusable element -->
<button aria-hidden="true">Submit</button>

<!-- Valid: aria-hidden on a non-focusable element -->
<div aria-hidden="true">decorative content</div>
```
