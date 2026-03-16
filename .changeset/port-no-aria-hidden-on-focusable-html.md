---
"@biomejs/biome": minor
---

# Port `noAriaHiddenOnFocusable` a11y lint rule to HTML

Added HTML support for the `noAriaHiddenOnFocusable` accessibility lint rule, which enforces that `aria-hidden="true"` is not set on focusable elements. Focusable elements include native interactive elements (`<button>`, `<a href>`, `<input>`, `<select>`, `<textarea>`, `<details>`, `<summary>`), elements with `tabindex >= 0`, and elements with `contenteditable="true"`. Includes an unsafe fix to remove the `aria-hidden` attribute.
