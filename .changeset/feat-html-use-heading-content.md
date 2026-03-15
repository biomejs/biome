---
"@biomejs/biome": minor
---

Added HTML support for the accessibility rule `useHeadingContent`.

This rule enforces that heading elements (h1-h6) have accessible content for screen readers. Headings must either contain text content that is not hidden with `aria-hidden`, or have an accessible name via `aria-label`, `aria-labelledby`, or `title` attributes.

The rule works in `.html` files (case-insensitive tag matching) as well as component-based frameworks like Vue, Svelte, and Astro (case-sensitive lowercase matching).
