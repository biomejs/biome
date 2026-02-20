---
"@biomejs/biome": patch
---

Fixed parsing of Svelte directive keywords (`use`, `style`) when used as plain text content in HTML/Svelte files. Previously, `<p>use JavaScript</p>` or `<p>style it</p>` would incorrectly produce a bogus element instead of proper text content.
