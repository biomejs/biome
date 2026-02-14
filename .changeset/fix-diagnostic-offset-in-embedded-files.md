---
"@biomejs/biome": patch
---

Fixed [#9057](https://github.com/biomejs/biome/issues/9057): Incorrect diagnostic spans for suppression comments and other raw diagnostics in HTML-ish files (Vue, Svelte, Astro). Previously, diagnostics like "unused suppression" pointed to the wrong location in the document due to the diagnostic offset not being applied.
