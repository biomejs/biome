## Summary

Ports the `useKeyWithClickEvents` lint rule from JSX to HTML, as part of the umbrella issue #8155.

## What the rule does

Enforces that elements with `onclick` handlers also have at least one keyboard event handler (`onkeydown`, `onkeyup`, or `onkeypress`) for accessibility.

### Exemptions
- **Inherently keyboard-accessible elements**: `<a>`, `<button>`, `<input>`, `<select>`, `<textarea>`, `<option>`
- **Elements hidden from assistive tech**: `aria-hidden="true"`, `type="hidden"`, `role="presentation"`/`role="none"`
- **Case handling**: In `.html` files, element names are matched case-insensitively. In component frameworks (Vue, Svelte, Astro), only lowercase names are checked (PascalCase assumed to be custom components).

## Test plan

- `valid.html`: onclick with keyboard handlers, inherently accessible elements, hidden elements, presentation roles, case-insensitive matching
- `invalid.html`: div/span/section/article/header/footer with onclick and no keyboard handler

Closes part of #8155.
