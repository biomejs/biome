---
"@biomejs/biome": minor
---

Added **experimental** full support for HTML, Vue, Svelte and Astro files. In this release, the HTML parser
has been enhanced, and it's now able to parse `.vue`, `.svelte` and `.astro` files.

This means that now Biome is able to lint and format the JavaScript (TypeScript), HTML and CSS code that is contained in these files.

Now that the main architecture is stable and working, in the upcoming patches and minors we will also fix possible inaccuracies and edge cases coming from existing lint rules, such as `noUnusedVariables` inside `<script>` blocks or frontmatter.

The support is considered experimental because there might be cases that aren't fine-parsed yet, hence causing possible inaccuracies when it comes to formatting and linting.
