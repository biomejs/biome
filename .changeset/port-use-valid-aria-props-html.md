---
"@biomejs/biome": minor
---

Port the `useValidAriaProps` lint rule to HTML. This rule checks that all `aria-*` attributes used in HTML elements are valid ARIA attributes as defined by the WAI-ARIA specification.

Previously this rule only worked with JSX. Now it also works with `.html`, `.astro`, `.vue`, and `.svelte` files.
