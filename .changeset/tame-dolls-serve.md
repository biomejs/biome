---
"@biomejs/biome": patch
---

Fixed an issue where Svelte globals ($state and so on) were not properly recognized inside `.svelte.test.ts/js` and `.svelte.spec.ts/js` files.
