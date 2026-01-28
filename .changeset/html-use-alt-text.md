---
"@biomejs/biome": minor
---

Added the [`useAltText`](https://biomejs.dev/linter/rules/use-alt-text/) lint rule for HTML. This rule enforces that elements requiring alternative text (`<img>`, `<area>`, `<input type="image">`, `<object>`) provide meaningful information for screen reader users via `alt`, `title` (for objects), `aria-label`, or `aria-labelledby` attributes. Elements with `aria-hidden="true"` are exempt.
