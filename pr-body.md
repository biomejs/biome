## Summary

Ports the `noAriaHiddenOnFocusable` lint rule from JSX to HTML, as part of the umbrella issue #8155.

## What the rule does

Enforces that `aria-hidden="true"` is not set on focusable elements. A focusable element with `aria-hidden` can be reached by keyboard but is invisible to screen readers, causing confusion.

### What counts as focusable
- Elements with non-negative `tabindex` (e.g., `tabindex="0"`)
- Natively focusable elements: `<button>`, `<input>`, `<select>`, `<textarea>`, `<details>`, `<summary>`, `<a href="...">`, `<area href="...">`
- Elements with `contenteditable="true"`

### Exemptions
- `aria-hidden="true"` on non-focusable elements (div, span, etc.) is valid
- `tabindex="-1"` with `aria-hidden="true"` is valid (intentionally removed from tab order)

### Fix
Provides an unsafe fix that removes the `aria-hidden` attribute.

## Test plan

- `valid.html`: aria-hidden on non-focusable elements, focusable elements without aria-hidden, negative tabindex with aria-hidden
- `invalid.html`: button/a/input/select/textarea with aria-hidden, tabindex="0" with aria-hidden, contenteditable with aria-hidden

Closes part of #8155.
