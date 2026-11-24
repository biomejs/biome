---
"@biomejs/biome": patch
---

Fixed range formatting in the editor for Vue, Svelte, and Astro files when full HTML support is disabled. Selections that end before the script block no longer format it or report an error, and edits for selections inside the script block are now applied at the selected location.
