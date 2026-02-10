---
"@biomejs/biome": patch
---

Fixed plugin diagnostics showing incorrect line numbers in Vue, Astro, and Svelte files. Plugin diagnostics now correctly account for the template/frontmatter offset, pointing to the right location in the `<script>` block.
