---
"@biomejs/biome": minor
---

Added the HTML version of the [`noLabelWithoutControl`](https://biomejs.dev/linter/rules/no-label-without-control/) rule. The rule now enforces that `<label>` elements have accessible text content and are associated with a form control in HTML, Vue, Svelte, and Astro files.

A label is valid when it either wraps a control (`input`, `select`, `textarea`, etc.) or has a `for` attribute pointing to a control's ID, and the label itself has non-empty text or an `aria-label`/`aria-labelledby` attribute.

```html
<!-- Invalid: empty label, no control -->
<label></label>

<!-- Invalid: has text but no control association -->
<label>A label</label>

<!-- Invalid: has `for` but no accessible text -->
<label for="name"></label>

<!-- Valid: text content + nested control -->
<label>Full name <input /></label>

<!-- Valid: `for` attribute + accessible text -->
<label for="name">Full name</label>
```
